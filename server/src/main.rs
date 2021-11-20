// Author : Quentin CHARLES, Nicolas TAHON, Tristan KLIEBER
// Projet de messagerie instantanée en Rust

use std::thread;
use std::time::Duration;
use std::io::prelude::*;

use std::net::{TcpListener, TcpStream};
use std::net::SocketAddr;
use std::net::TcpStream;


const ADDR: &str = "127.0.0.1:666";


fn main() {
    println!("first line");
    thread::sleep(Duration::from_secs(5));

    if let Ok(stream) = TcpStream::connect("127.0.0.1:8080") {
        println!("Connected to the server!");
    } else {
        println!("Couldn't connect to server...");
    }
}