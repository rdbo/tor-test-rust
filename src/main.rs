use arti_client::{BootstrapBehavior, TorClient};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> Result<(), arti_client::Error> {
    println!("Arti Test");
    let tor_client = TorClient::builder()
        .bootstrap_behavior(BootstrapBehavior::OnDemand)
        .create_unbootstrapped()?;
    println!("Created Tor Client");

    println!("Testing Tor Connection...");
    let mut stream = tor_client.connect(("example.com", 80)).await?;
    match stream.write_all(b"GET / HTTP/1.1\r\nHost: example.com\r\nConnection: close\r\n\r\n").await {
        Ok(_) => { println!("Written to stream successfully"); }
        Err(_) => { panic!("Failed to write to stream"); }
    }
    match stream.flush().await {
        Ok(_) => {  }
        Err(_) => { panic!("Failed to flush stream"); }
    }

    let mut buf = Vec::new();
    match stream.read_to_end(&mut buf).await {
        Ok(_) => { println!("Read response from stream"); }
        Err(_) => { panic!("Failed to read from stream"); }
    }

    println!("Response: {}", String::from_utf8_lossy(&buf));

    Ok(())
}
