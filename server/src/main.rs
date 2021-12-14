//Server
#![allow(unused_variables)]
#![allow(unused_imports)]

use tokio::{sync::broadcast, net::TcpListener, io::BufReader};
use tokio::io::AsyncWriteExt;
use tokio::io::AsyncBufReadExt;
use aes::Aes128;
use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;
use hex_literal::hex;
use tokio::io::AsyncReadExt;

//Adresse d'écoute
const LISTNER_ADDR: &str = "127.0.0.1:9999";


/*
fn dechiffrer (recv_data: &[u8]) -> String{
    type Aes128Cbc = Cbc<Aes128, Pkcs7>;
    let key = hex!("000102030405060708090a0b0c0d0e0f");
    let iv = hex!("f0f1f2f3f4f5f6f7f8f9fafbfcfdfeff");
    let cipher = Aes128Cbc::new_from_slices(&key, &iv).unwrap();

    let user_buffer_uncrypted = cipher.decrypt_vec(recv_data);
    let prefinal_buffer: &[u8] = &user_buffer_uncrypted.unwrap();

    let stdout_string:&str = std::str::from_utf8(prefinal_buffer).unwrap();
    stdout_string.to_string()
}
 */

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
            let recved_data: &mut [u8] = &mut [0u8];

            loop {
                //permet de lire et envoyer en même temps, un peu comme thread
                tokio::select! {
                    resultat_select = reading.read(recved_data) => {
                        let sended_data = &resultat_select.as_ref().unwrap();
                        println!("{}",sended_data);
                        println!("{:?}",&resultat_select);

                        //-------let mut recved_string = dechiffrer(recved_data);
                        //si on ne reçoit rien alors ça romp le TCPStream

                         //-------if recved_data == 0 {
                        //------- println!("Utilisateur {} s'est déconnecté", addr);
                         //------- break
                         //------- }


                        //si l'user entre "quit" alors ça romp le TCPStream
                        //-------if recved_string == String::from(":quit\n"){
                            //-------println!("Utilisateur {}, s'est déconnecté", addr);
                            //-------break
                        //-------}

                        tx.send((&sended_data, addr)).unwrap(); //-> rx (a besoin d'un &[u8]

                        //on enlève la touche "entré"
                        //-------if recved_string.ends_with('\n') { recved_string.pop(); if recved_string.ends_with('\r') { recved_string.pop();}}

                        //-------println!("{} à écrit : {}",addr, recved_string);
                        //-----TEMP   recved_data.clear();

                    }
                    resultat_select = rx.recv() => {
                        let (msguser, other_addresses) = resultat_select.unwrap();

                        if addr != other_addresses {
                            let sent_msguser = format!("{} a envoyé : {:?}", addr, msguser);
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
