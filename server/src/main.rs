//Server
#![allow(unused_variables)]
//#![allow(unused_imports)]

use tokio::{sync::broadcast, net::TcpListener, io::{BufReader, AsyncWriteExt, AsyncReadExt}};
use aes::Aes128;
use block_modes::{BlockMode, Cbc, block_padding::Pkcs7};
use hex_literal::hex;



//Adresse d'écoute
const LISTNER_ADDR: &str = "127.0.0.1:9999";


fn dechiffrer (recv_data: &[u8]) -> String{
    //cypher variables
    //----------------
    type Aes128Cbc = Cbc<Aes128, Pkcs7>;
    let key = hex!("000102030405060708090a0b0c0d0e0f");
    let iv = hex!("f0f1f2f3f4f5f6f7f8f9fafbfcfdfeff");
    let cipher = Aes128Cbc::new_from_slices(&key, &iv).unwrap();
    //---------------
    let user_buffer_uncrypted = cipher.decrypt_vec(recv_data).unwrap();
    let prefinal_buffer: &[u8] = &user_buffer_uncrypted;
    let stdout_string:&str = std::str::from_utf8(&prefinal_buffer).unwrap();
    stdout_string.to_string()
}

fn chiffrer (msguser: String) -> Vec<u8>{
    //cypher variables
    //----------------
    type Aes128Cbc = Cbc<Aes128, Pkcs7>;
    let key = hex!("000102030405060708090a0b0c0d0e0f");
    let iv = hex!("f0f1f2f3f4f5f6f7f8f9fafbfcfdfeff");
    let cipher = Aes128Cbc::new_from_slices(&key, &iv).unwrap();
    //---------------
    let msguser: &[u8] = &msguser.as_bytes();
    let user_buffer_encrypted = cipher.encrypt_vec(&msguser);
    user_buffer_encrypted
}

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
            let (reading, mut writing) = socket.split();

            let mut reading = BufReader::new(reading);
            let recved_data: &mut[u8] = &mut [0u8;2048];

            loop {
                tokio::select! {
                    resultat_select = reading.read(recved_data) => {
                        let buf_size = resultat_select.unwrap(); //nb d'élément du buffer reçu
                        //si on ne reçoit rien alors ça romp le TCPStream
                         if buf_size < 1 {
                            println!("Utilisateur {} s'est déconnecté", addr);
                            break
                         }

                        let new_slice = &recved_data[0..buf_size];
                        let mut recved_string = dechiffrer(&new_slice);

                        //si l'user entre "quit" alors ça romp le TCPStream
                        if recved_string == String::from(":quit\n"){
                            println!("Utilisateur {}, s'est déconnecté", addr);
                            break
                        }

                        //on enlève la touche "entré"
                        if recved_string.ends_with('\n') {
                            recved_string.pop();
                            if recved_string.ends_with('\r') {
                                recved_string.pop();}
                        }

                        println!("{} à écrit : {}",addr, recved_string);

                        tx.send((recved_string, addr)).unwrap(); //-> rx (a besoin d'un &[u8]
                    }
                    resultat_select = rx.recv() => {
                        let (msguser, other_addresses) = resultat_select.unwrap();
                        if addr != other_addresses {
                            let msguser_to_send = format!("{} a envoyé : {}", addr, msguser);

                            let crypted_msg = chiffrer(msguser_to_send.clone());
                            let crypted_msg = crypted_msg.as_slice();

                            //println!("slice avant envoie : {:?}\n", bytes_to_send);

                            writing.write_all(crypted_msg).await.unwrap();
                        }
                    }
                }
            }
        });
    }
}
