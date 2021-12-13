//Server

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
        //Accepte les clients qui viennent sur le TCPStream
        let (mut socket, addr) = listener.accept().await.unwrap();

        let tx = tx.clone();
        let mut rx = tx.subscribe();
        println!("Nouvelle connexion: {}", addr);

        tokio::spawn(async move {
            //nécessité de split le socket pour pouvoir écouter et écrire en même temps
            let (reading, mut writing) = socket.split();

            let mut reading = BufReader::new(reading);
            let mut recved_data = String::new();

            loop {
                //permet de lire et envoyer en même temps, un peu comme thread
                tokio::select! {
                    resultat_select = reading.read_line(&mut recved_data) => {
                        //si on ne reçoit rien alors ça romp le TCPStream
                        if resultat_select.unwrap() == 0 {
                            println!("Utilisateur {} s'est déconnecté", addr);
                            break
                        }
                        //si l'user entre "quit" alors ça romp le TCPStream
                        if recved_data == String::from(":quit\n"){
                            println!("Utilisateur {}, s'est déconnecté", addr);
                            break
                        }

                        tx.send((recved_data.clone(), addr)).unwrap(); //-> rx

                        //on enlève la touche "entré"
                        if recved_data.ends_with('\n') { recved_data.pop(); if recved_data.ends_with('\r') { recved_data.pop();}}

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
                /*
                match stream.read(&mut data) {
                    Ok(size) => {
                        //Si 0 octect reçu --> client déconnecté
                        if size < 1 {
                            println!("client {} déconnecté", adresse);
                            return
                        }
                        /*
                        // Ecrit dans la console et send le message au user distant
                        // OLD -> stream.write_all(&data[0..size]).unwrap();
                        tx.send(stream.read(&mut data).unwrap());
                        stream.write_all(&data[0..size]).unwrap();
                        let msguser = from_utf8(&data).unwrap();
                        println!("-> L'utilisateur {} à envoyé: {}\n", stream.peer_addr().unwrap(), msguser);
                        true
                         */
                    },
                    Err(ref e) if e.kind() == ErrorKind::WouldBlock => { return }
                    Err(e) => {
                        println!("Erreur I/O rencontré : {}", e);
                        return
                    },
                }{}
                 */
            }
        });
    }
}
/*
fn handle_client(mut stream: TcpStream, adresse: &str, tx: &dyn Send, rx: &dyn Send) {
    loop {
        let mut data = [0 as u8; 50]; // buffer 50 bytes
        match stream.read(&mut data) {
            Ok(size) => {
                //Si 0 octect reçu --> client déconnecté
                if size < 1 {
                    println!("client {} déconnecté", adresse);
                    return
                }
                /*
                // Ecrit dans la console et send le message au user distant
                // OLD -> stream.write_all(&data[0..size]).unwrap();
                tx.send(stream.read(&mut data).unwrap());
                stream.write_all(&data[0..size]).unwrap();
                let msguser = from_utf8(&data).unwrap();
                println!("-> L'utilisateur {} à envoyé: {}\n", stream.peer_addr().unwrap(), msguser);
                true
                 */
            },
            Err(ref e) if e.kind() == ErrorKind::WouldBlock => { return }
            Err(e) => {
                println!("Erreur I/O rencontré : {}", e);
                return
            },
        }{}
        // Ecrit dans la console et send le message au user distant
        // OLD -> stream.write_all(&data[0..size]).unwrap();
        tx.send(stream.read(&mut data).unwrap());
        stream.write_all(&data).unwrap();
        let msguser = from_utf8(&data).unwrap();
        println!("-> L'utilisateur {} à envoyé: {}\n", stream.peer_addr().unwrap(), msguser);
    }
}
*/
