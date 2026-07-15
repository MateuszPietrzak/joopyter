use zeromq::{Socket, SocketRecv, SocketSend};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Opening connection...");

    let mut socket = zeromq::ReqSocket::new();
    socket
        .connect("ipc:///tmp/joopyter-kernel-5")
        .await
        .expect("Failed to connect!");
    println!("Connection open");

    socket.send("ping".into()).await?;
    println!("Ping sent! Awaiting response...");
    let repl = socket.recv().await?;
    println!("Response received: {:?}", repl);

    Ok(())
}
