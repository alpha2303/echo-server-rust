mod thread;

const HOST: &str = "127.0.0.1";
const PORT: u32 = 7;
const BUFFER_SIZE: usize = 4096;

enum ConnectionType {
    TCP,
    UDP,
}

use std::env;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream, UdpSocket};
use std::str::from_utf8;
use thread::{threadpool::ThreadPool, threadpool_error::ThreadPoolError};

fn handle_tcp_connection(mut conn_stream: TcpStream) {
    println!(
        "Accepted connection from: {:?}",
        conn_stream.peer_addr().unwrap()
    );

    conn_stream
        .write(b"Welcome to the echo server!\r\n")
        .unwrap();

    loop {
        let mut buffer = [0; BUFFER_SIZE];
        match conn_stream.read(&mut buffer) {
            Ok(size) => {
                let input_str: &str = from_utf8(&buffer[..size]).unwrap().trim_end();
                if input_str == "\u{3}" || size == 0 {
                    println!("Client Disconnected");
                    break;
                };
                println!("Received: {:?}", input_str);
                conn_stream
                    .write(format!("{}\r\n", input_str).as_bytes())
                    .unwrap();
            }
            Err(e) => {
                eprintln!("Failed to read from stream: {e}");
                break;
            }
        }
    }
}

fn start_tcp_server(address: String, conn_pool: ThreadPool) {
    let listener: TcpListener = TcpListener::bind(&address).unwrap();

    println!("Server will run on: {}", address);
    loop {
        for stream in listener.incoming() {
            match stream {
                Ok(_stream) => {
                    conn_pool.execute(|| {
                        handle_tcp_connection(_stream);
                    });
                    break;
                }
                Err(e) => {
                    eprintln!("Failed connection: {e}");
                }
            }
        }
    }
}

fn start_udp_server(address: String, conn_pool: ThreadPool) {
    let socket = match UdpSocket::bind(&address) {
        Ok(socket) => socket,
        Err(e) => {
            eprintln!("Failed to bind UDP socket: {e}");
            return;
        }
    };

    println!("UDP Echo server listening on: {}", address);
    let mut buf: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];

    loop {
        println!("Waiting for data...");
        buf.fill(0); // Clear buffer before each read
        match socket.recv_from(&mut buf) {
            Ok((size, source_addr)) => {
                let received_data = buf[..size].to_vec();
                let socket_clone = socket.try_clone().unwrap();

                conn_pool.execute(move || {
                    let input_str = String::from_utf8_lossy(&received_data);
                    println!("Received from {}: {:?}", source_addr, input_str);

                    if let Err(e) = socket_clone.send_to(received_data.as_slice(), source_addr) {
                        eprintln!("Failed to send response to {}: {}", source_addr, e);
                    }
                });
            }
            Err(e) => {
                eprintln!("Failed to read from socket: {e}");
            }
        }
    }
}

fn main() {
    let address: String = format!("{}:{}", HOST, PORT);
    let mut conn_type = ConnectionType::TCP;
    let args: Vec<String> = env::args().collect();
    println!("Arguments: {:?}", args);

    if args.len() == 2 && args.last() == Some(&"--udp".to_string()) {
        println!("Using UDP Protocol");
        conn_type = ConnectionType::UDP;
    } else {
        println!("Defaulting to TCP Protocol");
    }

    let pool_result: Result<ThreadPool, ThreadPoolError> = ThreadPool::new(4);
    if let Err(e) = pool_result {
        eprintln!("Failed to create thread pool: {e}");
        return;
    }
    let conn_pool: ThreadPool = pool_result.unwrap();

    match conn_type {
        ConnectionType::TCP => start_tcp_server(address, conn_pool),
        ConnectionType::UDP => start_udp_server(address, conn_pool),
    }
}
