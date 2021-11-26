// Author : Quentin CHARLES, Nicolas TAHON, Tristan KLIEBER                                                                                                                                                                                                                          
// Projet de messagerie instantanée en Rust                                                                                                                                                                                                                                          
                                                                                                                                                                                                                                                                                     
use std::thread;                                                                                                                                                                                                                                                                     
use std::time::Duration;                                                                                                                                                                                                                                                             
use std::net::{TcpListener, TcpStream};                                                                                                                                                                                                                                              
use std::sync::mpsc::channel;                                                                                                                                                                                                                                                        
                                                                                                                                                                                                                                                                                     
//Adresse d'écoute                                                                                                                                                                                                                                                                   
const LISTNER_ADDR: &str = "127.0.0.1:9999";                                                                                                                                                                                                                                         
                                                                                                                                                                                                                                                                                     
fn main() {                                                                                                                                                                                                                                                                          
    //lancement du TCPListener et écoute continue du socket                                                                                                                                                                                                                          
    let server = TcpListener::bind(LISTNER_ADDR).expect("TCPListener échoué");                                                                                                                                                                                                       
    server.set_nonblocking(true).expect("impossible de mettre la connexion en 'nonblocking'");                                                                                                                                                                                       
                                                                                                                                                                                                                                                                                     
    //Initialise client vector macro list
    let client = vec![];


    //Création de la channel de chat
    //https://doc.rust-lang.org/std/sync/mpsc/
    let (tx, rx) = channel();
    thread::spawn(move|| {
        tx.send(10).unwrap();
    });
    assert_eq!(rx.recv().unwrap(), 10);

    //boucle de réception + affichage des messages
    loop {


    }

}
