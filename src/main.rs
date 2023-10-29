use std::error::Error;
mod ast;
use ast::Token;
use ast::Input;
use ast::SocketState;

use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("127.0.0.1:9999").await?;

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let mut buf: [u8; 1024] = [0; 1024];

            let n = match socket.read(&mut buf).await {
                // socket closed
                Ok(n) if n == 0 => return,
                Ok(n) => n,
                Err(e) => {
                    eprintln!("failed to read from socket; err = {:?}", e);
                    return;
                }
            };
            
            let mut input = Input::new(&buf, n);
            let token_result = input.analysis();

            let token: Token = match token_result {
                Err(_err) => {
                    let _ = socket.write("Please use the correct command! \r\n".as_bytes()).await;
                    let _ = socket.shutdown().await;
                    return ();
                },
                Ok(token) => token,
            };
            //let token: Token = token_result.unwrap();

            let _r = match token.state {
                SocketState::CreateTopic => {
                    let _ = socket.write("run ok! \r\n".as_bytes()).await;
                    socket.shutdown().await
                },
                SocketState::DeleteGroup | 
                SocketState::Alter | 
                SocketState::DeleteTopic | 
                SocketState::Describe | 
                SocketState::GetOffset | 
                SocketState::List | 
                SocketState::None => {
                    let _ = socket.write("run ok! \r\n".as_bytes()).await;
                    socket.shutdown().await
                },
                _ => Ok(()),
            };

            // In a loop, read data from the socket and write the data back.
            loop {

                let n = match socket.read(&mut buf).await {
                    // socket closed
                    Ok(n) if n == 0 => return,
                    Ok(n) => n,
                    Err(e) => {
                        eprintln!("failed to read from socket; err = {:?}", e);
                        return;
                    }
                };

                println!("string: {:?}", String::from_utf8((&buf[0..n]).to_vec()).unwrap());

                //socket.shutdown().await;
                // Write the data back
                // if let Err(e) = socket.write_all(&buf[0..n]).await {
                //     eprintln!("failed to write to socket; err = {:?}", e);
                //     return;
                // }
            }
        });
    }

}