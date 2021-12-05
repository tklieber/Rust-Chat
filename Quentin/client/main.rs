//Client

use std::net::{TcpStream};
use std::io::{Read, Write};
use std::str::from_utf8;
use std::io;



fn user_input() -> String {
    let mut buffer = String::new();
    let stdin = io::stdin();
    match stdin.read_line(&mut buffer){
        Ok(_) => {
            println!("entrer: {}", buffer);
        },
        Err(e) => println!("Une erreur est survenue : {}", e)
    }

    return buffer;
}



fn main() {
    match TcpStream::connect("localhost:3333") {
        Ok(mut stream) => {
            println!("Connexion réussi sur le port 3333");
/*
            let mut buffer = String::new();
            let mut stdin = io::stdin();
            stdin.read_line(&mut buffer);
            println!("entrer: {}", buffer)?;
            Ok(())
*/





            loop{
            let input: String;
            input = user_input();
            let msg = input.as_bytes();
            let msglen = msg.len();
           //let msg = buffer.as_bytes();
            //let msg = b"Ping!!";

            stream.write(msg).unwrap();
            println!("Message envoyé, en attente de la réponse...");

            //let mut data = [0 as u8; 25]; // buffer 6 bytes
            let mut data: Vec<u8> = vec![0; msglen];
            match stream.read_exact(&mut data) {
                Ok(_) => {
                    if &data == msg {
                        println!("Message reçu!");
                        let msgreceive = from_utf8(&data).unwrap();
                        println!("le message reçu est {}", msgreceive);

                    } else {
                        let text = from_utf8(&data).unwrap();
                        println!("Réponse unexpected: {}", text);
                    }
                },
                Err(e) => {
                    println!("Erreur dans la réception des données: {}", e);
                }
            }
            }
        },
        Err(e) => {
            println!("Erreur de connexion: {}", e);
        }
            }   
    println!("Connexion terminé");
}
