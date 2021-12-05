//Server

use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};
use std::str::from_utf8;

fn handle_client(mut stream: TcpStream) {
    loop{
    let mut data = [0 as u8; 50]; // buffer 50 bytes

    while match stream.read(&mut data) {
        Ok(size) => {
            // Ecrit dans la console et send le message au user distant
            stream.write(&data[0..size]).unwrap();

            let msguser = from_utf8(&data).unwrap();
            println!("-> L'utilisateur {} à envoyé: {}\n", stream.peer_addr().unwrap(), msguser);
            true
        },
        Err(_) => {
            println!("Une erreur est survenu, connexion interrompu {}", stream.peer_addr().unwrap());
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}
}



fn main() {
    let listener = TcpListener::bind("0.0.0.0:3333").unwrap();
    // accepte les connection, et crée un thread nouveau thread pour chacune d'entre elle
 
    println!("Le server écoute sur le port 3333");
    /*
    let addrs = [
    SocketAddr::from(([127, 0, 0, 1], 8080)),
    SocketAddr::from(([127, 0, 0, 1], 8081)),
];*/

//watch this -> https://stackoverflow.com/questions/60219160/how-to-store-tcpstream-inside-a-hashmap
    for stream in listener.incoming() {
        //vector to store clients
        let mut clients = vec![];
        match stream {
            Ok(stream) => {
                println!("Nouvelle connexion: {}", stream.peer_addr().unwrap());
                //push client into vector
                clients.push(socket.try_clone().expect("failed to clone client"));
                thread::spawn(move|| {
                    // connexion réussi
                    handle_client(stream)
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
