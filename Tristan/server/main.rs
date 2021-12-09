//Server

use std::thread;
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write, ErrorKind};
use std::str::from_utf8;

//Adresse d'écoute
const LISTNER_ADDR: &str = "127.0.0.1:9999";

fn handle_client(mut stream: TcpStream, adresse: &str, clients: &Vec<TcpStream>) {
    loop {
        let mut data = [0 as u8; 50]; // buffer 50 bytes
        //let msguser:Vec<u8> = Vec::new();


        while match stream.read(&mut data) {
            Ok(size) => {
                //Si 0 octect reçu --> client déconnecté
                if size < 1 {
                    println!("client déconnecté {}", adresse);
                    return
                }
                // Ecrit dans la console et send le message au user distant
                stream.write_all(&data[0..size]).unwrap();

                let msguser = from_utf8(&data).unwrap();
                println!("-> L'utilisateur {} à envoyé: {}\n", stream.peer_addr().unwrap(), msguser);
                true
            },
            Err(ref e) if e.kind() == ErrorKind::WouldBlock => { false }
            Err(e) => {
                println!("Erreur I/O rencontré : {}", e);
                false
            },
        }{}
    }
}



fn main() {
    let listener = TcpListener::bind(LISTNER_ADDR).expect("TCPListener échoué");
    // accepte les connection, et crée un thread nouveau thread pour chacune d'entre elle
    println!("écoute sur le port 3333....");


//watch this -> https://stackoverflow.com/questions/60219160/how-to-store-tcpstream-inside-a-hashmap
    for stream in listener.incoming() {
        //vector to store clients
        let mut clients = Vec::new();
        match stream {
            Ok(stream) => {
                let adresse = match stream.peer_addr() {
                    Ok(addr) => format!("[adresse : {}]", addr),
                    Err(_) => "inconnue".to_owned()
                };
                println!("Nouvelle connexion: {}", stream.peer_addr().unwrap());
                clients.push(stream.try_clone().expect("failed to clone client"));

                thread::spawn(move|| {
                    // connexion réussi
                    handle_client(stream, &*adresse, &clients)
                });
            }
            Err(e) => {
                println!("Erreur: {}", e);
                /* connexion échoué */
            }
        }

    }
    // close sockets
    drop(listener);
}
