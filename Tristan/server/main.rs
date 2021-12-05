// Author : Quentin CHARLES, Nicolas TAHON, Tristan KLIEBER                                                                                                                                                                                                                          
// Projet de messagerie instantanée en Rust                                                                                                                                                                                                                                          

use std::thread;
use std::time::Duration;
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc::channel;

//Adresse d'écoute
const LISTNER_ADDR: &str = "127.0.0.1:9999";

//fn send_message(&mut String) {

//}

fn main() {
    //lancement du TCPListener et écoute continue du socket
    let server = TcpListener::bind(LISTNER_ADDR).expect("TCPListener échoué");
    server.set_nonblocking(true).expect("impossible de mettre la connexion en 'nonblocking'");

    //Initialise client vector macro list
    let mut client = vec![];

    //Création de la channel de chat
    //Voir --> https://doc.rust-lang.org/std/sync/mpsc/
    let (tx, rx) = channel();

    //boucle de réception + affichage des messages
    loop {
        //DOC --> https://doc.rust-lang.org/std/net/struct.TcpListener.html
        if let Ok((mut _socket, addr)) = server.accept() {
            println!("client {:?} connected !", addr);

            let tx = tx.clone();
            client.push(_socket.try_clone().expect("impossible de cloner le client dans vec![]"));


            //Envoie de message sur le flux de transmission
            thread::spawn(move || {
                let msg = String::from("hello world !");
                let _sending = match tx.send(msg) {
                    Ok(sending) => sending,
                    Err(_error) => panic!("message cannot be send"),
                };
                thread::sleep(Duration::from_millis(10));
            });
            while let Ok(msg) = rx.recv() {
                println!("{}", msg);
            }


            thread::sleep(Duration::from_millis(10));
        }
    }

}
