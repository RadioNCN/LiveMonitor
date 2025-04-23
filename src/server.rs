use std::collections::HashMap;
use std::sync::Arc;
use tokio::io::{BufReader, AsyncBufReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{mpsc, Mutex};
use tokio::runtime::Runtime;

pub(crate) async fn ConnectionManager(tx: mpsc::Sender<(String,(f64,f64))>){
    let server_address = "127.0.0.1:7800";
    let data_max_len = 1000;
    let output = Arc::new(Mutex::new(tx));

    match TcpListener::bind(server_address).await {
        Ok(listener) => {
            loop {
                match listener.accept().await {
                    Ok((stream, socket)) => {
                        let output = Arc::clone(&output);
                        tokio::spawn(async move {
                            let mut reader = BufReader::new(stream);
                            let mut buffer = String::new();
                            let mut msg2read = 0i8;
                            let mut rx_msg = (0f64, 0f64);
                            let mut stream_id = "temp".to_string();
                            while let Ok(bytes_read) = reader.read_line(&mut buffer).await{
                                if bytes_read == 0 {println!("Connection was closed"); break;}
                                match msg2read {
                                    0 => {
                                        match buffer.trim().parse::<String>() {
                                            Ok(value) => {
                                                println!("received Name: {value}");
                                                stream_id = value;
                                            }
                                            Err(e) => {println!("could not parse: {}", e)}
                                        }
                                        msg2read = 1;
                                    }
                                    1 => {
                                        match buffer.trim().parse::<f64>() {
                                            Ok(value) => {
                                                rx_msg.0 = value;
                                            }
                                            Err(e) => {println!("could not parse: {}", e)}
                                        }
                                        msg2read = 2;
                                    }
                                    2 => {
                                        match buffer.trim().parse::<f64>() {
                                            Ok(value) => {
                                                rx_msg.1 = value;
                                                let mut output_lock = output.lock().await;
                                                let id =stream_id.clone();
                                                output_lock.send((id, rx_msg)).await;

                                                drop(output_lock)
                                            }
                                            Err(e) => {println!("could not parse: {}", e)}
                                        }
                                        msg2read = 1;
                                    }
                                    _ => {}
                                }
                                buffer.clear()
                            }
                        });
                    }
                    Err(e) => {println!("Can't connect: {}", e)}
                }
            }
        }
        Err(e) => {println!("Can't bind: {}", e)}
    }

}