use tokio::{io::{AsyncReadExt, AsyncWriteExt}, time::sleep};
use tokio_serial::{SerialPortBuilderExt, SerialStream, DataBits, Parity, StopBits};
use std::sync::Arc;

struct SharedBuffer{

    buff : Arc<tokio::sync::Mutex<[u8; 1024]>>,
    len : Arc<tokio::sync::Mutex<usize>>,
}

impl  SharedBuffer {
    
    fn new() -> Self {
        SharedBuffer {
            buff: Arc::new(tokio::sync::Mutex::new([0; 1024])),
            len: Arc::new(tokio::sync::Mutex::new(0)),
        }
    }

}


#[tokio::main]
async fn main()-> Result<(),Box<dyn std::error::Error>> {
    
    let port = async_serial_open("com1", 921600, 1000).await.expect("Busy Port");
    let (mut reader, mut writer) = tokio::io::split(port); // Use One Port In Any Task 

    let memory = SharedBuffer::new();

    let sendr_task = async {
            
        let mem: Arc<tokio::sync::Mutex<[u8; 1024]>> = Arc::clone(&memory.buff);
        let len: Arc<tokio::sync::Mutex<usize>> = Arc::clone(&memory.len);
        loop {

            if  *len.lock().await > 0 {

                writer.write_all(&mem.lock().await[..*len.lock().await]).await.expect("Write Error");
                *len.lock().await = 0; // Reset length after sending
            }
            // sleep(tokio::time::Duration::from_millis(500)).await;
        }
    };

    let reciver_task = async {

        let mem: Arc<tokio::sync::Mutex<[u8; 1024]>> = Arc::clone(&memory.buff);
        let len: Arc<tokio::sync::Mutex<usize>> = Arc::clone(&memory.len);
        let mut buff: [u8; 1024] = [0; 1024];
        loop {
            
            let n  = {

                match reader.read( &mut buff).await {

                    Ok(n) if n > 0 => n,

                    Ok(_) => 0 as usize,

                    Err(e) => {

                        println!("{}", e);
                        0 as usize

                    }
                }
            };

            if n > 0 {
                
                mem.lock().await[..n].copy_from_slice(&buff[..n]);
                *len.lock().await = n;
                println!("Length Received: {}", n);
                println!("Buffer: {:?}", std::str::from_utf8(&mem.lock().await[..n]).unwrap_or("Invalid UTF-8"));  //convert row buffer to string
            }
        
        };
    };


    tokio::select! {

        _ = sendr_task => println!("Stop Sender"),
        _ = reciver_task => println!("Stop Reader"),

    }



    Ok(())
}

/**
 * 
 */
async fn async_serial_open(port_name: &str, baud: u32, timeout_ms: u64) -> Result<SerialStream, Box<dyn std::error::Error>> {

  let port: SerialStream = tokio_serial::new(port_name, baud)
    .data_bits(DataBits::Eight)
    .baud_rate(baud)
    .parity(Parity::None)
    .stop_bits(StopBits::One)
    .timeout(tokio::time::Duration::from_millis(timeout_ms))
    .open_native_async()?;

    Ok(port)
}