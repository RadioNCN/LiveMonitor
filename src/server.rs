use std::collections::HashMap;
use std::sync::Arc;
use dashmap::DashMap;
use tokio::io::{BufReader, AsyncBufReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{mpsc, Mutex};
use tokio::runtime::Runtime;
use crate::{pltGraph, pltHeatmap};

pub(crate) async fn GraphServer(db: Arc<DashMap<String,Vec<(f64, f64)>>>,
                                para: Arc<DashMap<String, pltGraph::Plotpara>>,
                                data_cap: Arc<Mutex<usize>>) {
    let server_address = "127.0.0.1:7800";
    match TcpListener::bind(server_address).await {
        Ok(listener) => {
            loop {
                match listener.accept().await {
                    Ok((stream, socket)) => {
                        let db = Arc::clone(&db);
                        let para = Arc::clone(&para);
                        let cap = Arc::clone(&data_cap);
                        tokio::spawn(async move {
                            let mut reader = BufReader::new(stream);
                            let mut buffer = String::new();
                            let mut msg2read = 0i8;
                            let mut rx_msg = (0f64, 0f64);
                            let mut stream_id = "temp".to_string();
                            while let Ok(bytes_read) = reader.read_line(&mut buffer).await {
                                if bytes_read == 0 {
                                    println!("Connection was closed");
                                    break;
                                }
                                match msg2read {
                                    0 => {
                                        match buffer.trim().parse::<String>() {
                                            Ok(value) => {
                                                println!("received Name: {value}");
                                                stream_id = value;
                                                if db.contains_key(&stream_id) == false {
                                                    let _ = db.insert(stream_id.clone(), vec![]);
                                                    let _ = para.insert(stream_id.clone(), pltGraph::Plotpara::default());
                                                }
                                            }
                                            Err(e) => { println!("could not parse: {}", e) }
                                        }
                                        msg2read = 1;
                                    }
                                    1 => {
                                        match buffer.trim().parse::<f64>() {
                                            Ok(value) => {
                                                rx_msg.0 = value;
                                            }
                                            Err(e) => { println!("could not parse: {}", e) }
                                        }
                                        msg2read = 2;
                                    }
                                    2 => {
                                        match buffer.trim().parse::<f64>() {
                                            Ok(value) => {
                                                rx_msg.1 = value;
                                                if let Some(mut entry) = db.get_mut(&stream_id) {
                                                    entry.push(rx_msg);
                                                    let cap_lock = cap.lock().await;
                                                    while entry.len() > *cap_lock {
                                                        entry.remove(0);
                                                    }
                                                    drop(cap_lock);
                                                }
                                            }
                                            Err(e) => { println!("could not parse: {}", e) }
                                        }
                                        msg2read = 1;
                                    }
                                    _ => {}
                                }
                                buffer.clear()
                            }
                        });
                    }
                    Err(e) => { println!("Can't connect: {}", e) }
                }
            }
        }
        Err(e) => { println!("Can't bind: {}", e) }
    }
}

pub(crate) async fn HeatmapServer(db: Arc<DashMap<String,Vec<(f64, f64)>>>,
                                  para: Arc<DashMap<String, pltHeatmap::Plotpara>>) {
    let server_address = "127.0.0.1:7810";
    match TcpListener::bind(server_address).await {
        Ok(listener) => {
            loop {
                match listener.accept().await {
                    Ok((stream, socket)) => {
                        let db = Arc::clone(&db);
                        let para = Arc::clone(&para);
                        tokio::spawn(async move {
                            let mut reader = BufReader::new(stream);
                            let mut buffer = String::new();
                            let mut msg2read = 0i8;
                            let mut rx_msg = vec![];
                            let mut stream_id = "temp".to_string();
                            let len =50;
                            let mut i=0;
                            while let Ok(bytes_read) = reader.read_line(&mut buffer).await {
                                if bytes_read == 0 {
                                    println!("Connection was closed");
                                    break;
                                }
                                match msg2read {
                                    0 => {
                                        match buffer.trim().parse::<String>() {
                                            Ok(value) => {
                                                println!("received Name: {value}");
                                                stream_id = value;
                                                if db.contains_key(&stream_id) == false {
                                                    let _ = db.insert(stream_id.clone(), vec![]);
                                                    let _ = para.insert(stream_id.clone(), pltHeatmap::Plotpara::default());
                                                }
                                            }
                                            Err(e) => { println!("could not parse: {}", e) }
                                        }
                                        msg2read = 1;
                                    }
                                    1 => {
                                        match buffer.trim().parse::<f64>() {
                                            Ok(value) => {
                                                println!("{value}");
                                                rx_msg.push((value, i as f64));
                                            }
                                            Err(e) => { println!("could not parse: {}", e) }
                                        }
                                        i+=1;
                                        if i ==len -1 {msg2read = 2};
                                    }
                                    2 => {
                                        match buffer.trim().parse::<f64>() {
                                            Ok(value) => {
                                                rx_msg.push((value, i as f64));
                                                i=0;
                                                if let Some(mut entry) = db.get_mut(&stream_id) {
                                                    entry.clear();
                                                    entry.append(&mut rx_msg);
                                                }
                                            }
                                            Err(e) => { println!("could not parse: {}", e) }
                                        }
                                        msg2read = 1;
                                    }
                                    _ => {}
                                }
                                buffer.clear()
                            }
                        });
                    }
                    Err(e) => { println!("Can't connect: {}", e) }
                }
            }
        }
        Err(e) => { println!("Can't bind: {}", e) }
    }
}