//Server

use std::thread;
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write, ErrorKind};
use std::str::from_utf8;

//Adresse d'écoute
const LISTNER_ADDR: &str = "0.0.0.0:9969";

struct User {
    username: String,
    address: String,
}

fn handle_client(mut stream: TcpStream, adresse: &str, user_addr: String) {
    let returned = define_user(stream, &*adresse, &*user_addr);
    loop {
        let mut data = [0 as u8; 50]; // buffer 50 bytes

        while match stream.read(&mut data) {
            Ok(size) => {
                //Si 0 octect reçu --> client déconnecté
                if size < 1 {
                    println!("client déconnecté {}", adresse);
                    return
                }
                // Ecrit dans la console et send le message au user distant
                stream.write(&data[0..size]).unwrap();

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

fn define_user(mut stream: TcpStream, formatted_address: &str, adresse: &str) -> bool{
    let mut data = [0 as u8; 50]; //define buffer

    //demande le pseudo qui doit etre utilisé
    let ask_pseudo: String = String::from("Entrez le pseudo que vous voulez utiliser");
    let asking = ask_pseudo.as_byte();
    stream.write(asking).expect("erreur lors de l'envoie du message dans le stream");

    let returned: bool = false;
    match stream.read(&mut data) {
        Ok(size) => {
            //Si 0 octet reçu --> client déconnecté
            if size < 1 {
                println!("client déconnecté {}", adresse);
                returned = false;
            }
            // Ecrit dans la console et send le message au user distant
            stream.write(&data[0..size]).unwrap();
            let username = from_utf8(&data).unwrap();
            println!("-> L'utilisateur {} à envoyé le pseudo suivant: {}\n", stream.peer_addr().unwrap(), username);

            returned = true;
        },
        Err(ref e) if e.kind() == ErrorKind::WouldBlock => { returned = false; }
        Err(e) => {
            println!("Erreur I/O rencontré : {}", e);
            returned = false;
        },
    }{}
    returned
}


fn main() {
    let listener = TcpListener::bind(LISTNER_ADDR).expect("TCPListener échoué");
    //listener.set_nonblocking(true).unwrap();
    println!("écoute sur le port 3333....");

//watch this -> https://stackoverflow.com/questions/60219160/how-to-store-tcpstream-inside-a-hashmap
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let formatted_address = match stream.peer_addr() {
                    Ok(addr) => {
                        format!("[adresse : {}]", addr)
                    },
                    Err(_) => "inconnue".to_owned()
                };

                let adresse = match stream.peer_addr() {
                    Ok(addr) => addr,
                    Err(e) => {println!("Erreur : {}", e);}
                };

                println!("Nouvelle connexion: {}", stream.peer_addr().unwrap());

                //handle client
                thread::spawn(move|| {
                    handle_client(stream, &*formatted_address, &*adresse)
                });
            }
            Err(e) => {
                println!("Erreur: {}", e);
            }
        }
    }
    // close sockets
    drop(listener);
}
