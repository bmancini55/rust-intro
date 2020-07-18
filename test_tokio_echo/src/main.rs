use futures::stream::StreamExt;
use tokio::net::TcpListener;
use tokio::prelude::*;

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:6142";
    let mut listener = TcpListener::bind(addr).await.unwrap();

    // Here we convert the `TcpListener` to a stream of incoming
    // connections with the `incoming` method
    let server = async move {
        // Incoming creates a stream from listener that returns sockets
        // as they come in.
        let mut incoming = listener.incoming();

        // Use next() from futures::stream::StreamExt
        while let Some(socket_res) = incoming.next().await {
            match socket_res {
                Ok(mut socket) => {
                    println!("Accepted connection from {:?}", socket.peer_addr());
                    // Spawn the future that echos the data and returns
                    // how many bytes were copied as a concurrent task.
                    tokio::spawn(async move {
                        // Split up the reading and writing parts of the
                        // socket using the .split method
                        let (mut reader, mut writer) = socket.split();

                        // Use the IO copy function which copies the
                        // data from the reader into the writer.
                        match tokio::io::copy(&mut reader, &mut writer).await {
                            Ok(amt) => {
                                println!("wrote {} bytes", amt);
                            }
                            Err(err) => {
                                eprintln!("IO error {:?}", err);
                            }
                        }
                    });
                }
                Err(err) => {
                    // Handle error by printing to STDOUT
                    println!("accept error = {:?}", err);
                }
            }
        }
    };

    println!("Server running on localhost:6142");

    server.await;
}
