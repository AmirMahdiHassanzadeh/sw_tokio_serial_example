use tokio_serial::{SerialPortBuilderExt, SerialStream, DataBits, Parity, StopBits};
use tokio::io::{AsyncWriteExt, AsyncReadExt};

#[tokio::main]
async fn main()-> Result<(),Box<dyn std::error::Error>> {
    
    let port = async_serial_open("com1", 921600, 1000).await.expect("Busy Port");

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