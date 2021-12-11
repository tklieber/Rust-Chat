//Server

//use std::io::{Read, Write, ErrorKind};
use tokio::{sync::broadcast, net::TcpListener, io::BufReader};
use tokio::io::AsyncWriteExt;
use tokio::io::AsyncBufReadExt;



//Adresse d'écoute
const LISTNER_ADDR: &str = "127.0.0.1:9999";



#[tokio::main]
async fn main() {
    let listener = TcpListener::bind(LISTNER_ADDR).await.unwrap();
    println!("En attente d'un client....");

    let (tx, _rx) = broadcast::channel(10);

    loop {
        //Accept les clients qui viennent sur le TCPStream
        let (mut socket, addr) = listener.accept().await.unwrap();

        let tx = tx.clone();
        let mut rx = tx.subscribe();
        println!("Nouvelle connexion: {}\n", addr);

        tokio::spawn(async move {
            //handle_client(stream, &*adresse, &tx, &rx);
            //nécessité de split le socket pour pouvoir écouter et écrire en même temps
            let (reading, mut writing) = socket.split();

            let mut reading = BufReader::new(reading);
            let mut recved_data = String::new();
            // buffer -> Obligé que ça soit un String car .clone()

            loop {
                //permet de lire et envoyer en même temps, un peu comme thread
                tokio::select! {
                    resultat_select = reading.read_line(&mut recved_data) => {
                        //si on reçoit rien -> break
                        if resultat_select.unwrap() == 0 { break }

                        tx.send((recved_data.clone(), addr)).unwrap();
                        println!("{} à écrit : {}",addr, recved_data);
                        recved_data.clear();
                    }
                    resultat_select = rx.recv() => {
                        let (msguser, other_addresses) = resultat_select.unwrap();

                        if addr != other_addresses {
                            let sent_msguser = format!("{} a envoyé : {}", addr, msguser);
                            writing.write_all((sent_msguser).as_bytes()).await.unwrap()
                        }
                    }
                }
