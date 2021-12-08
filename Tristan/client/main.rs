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
            //Enleve la touche 'Entrer'
            if buffer.ends_with('\n') {
                buffer.pop();
                if buffer.ends_with('\r') {
                    buffer.pop();
                }
            }
            println!("> entré : {}", buffer)
        },
        Err(e) => println!("Une erreur est survenue : {}", e)
    }
    buffer
}

fn send_and_receive_msg(mut stream: &TcpStream){
    loop{
        let input: String = user_input();
        let msg = input.as_bytes();
        let msglen = msg.len();

        stream.write(msg).expect("erreur lors de l'envoie du message dans le stream");
        print!("Message envoyé, en attente de la réponse.....");

        //let mut data = [0 as u8; 25]; // buffer 6 bytes
        let mut data: Vec<u8> = vec![0; msglen];

        match stream.read_exact(&mut data) {
            Ok(_) => {
                if &data == msg {
                    println!("Message reçu!");
                    //let msgreceive = from_utf8(&data).unwrap();
                    //println!("le message reçu est {}", msgreceive);
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
}

fn main() {
    match TcpStream::connect("localhost:3333") {
        Ok(mut stream) => {
            println!("Connexion réussi sur le port 3333 !");
            send_and_receive_msg(&mut stream);
        },
        Err(e) => {
            println!("Erreur de connexion: {}", e);
        }
            }   
    println!("Connexion terminé");
}
