//
// Rust : Projet de messagerie instantanée
//
// Auteurs : Quentin CHARLES, Nicolas TAHON, Tristan KLIEBER
//

use std::process;
use tokio::{sync::broadcast,
            net::TcpListener,
            io::{BufReader, AsyncWriteExt, AsyncReadExt}};
use aes::Aes128;
use block_modes::{BlockMode, Cbc,
                  block_padding::Pkcs7};
use hex_literal::hex;
use std::str;


//Adresse d'écoute
const LISTNER_ADDR: &str = "127.0.0.1:9999";

struct Program {
    name: String
}

impl Program {
    fn new(name: String) -> Program {
        Program { name }
    }

    fn print_error(&self,mesg: String) {
        println!("{}: Erreur rencontré : {}",self.name ,mesg);
    }

    fn print_fail(&self,mesg: String) -> ! {
        self.print_error(mesg);
        self.fail();
    }

    fn exit(&self,status: i32) -> ! { process::exit(status); }
    fn fail(&self) -> ! { self.exit(-1); }
}


fn dechiffrer (recv_data: &[u8]) -> String{
    //cypher variables
    //----------------
    type Aes128Cbc = Cbc<Aes128, Pkcs7>;
    let key = hex!("000102030405060708090a0b0c0d0e0f");
    let iv = hex!("f0f1f2f3f4f5f6f7f8f9fafbfcfdfeff");
    let cipher = Aes128Cbc::new_from_slices(&key, &iv).unwrap();
    //---------------
    let user_buffer_uncrypted = cipher.decrypt_vec(recv_data).unwrap();
    let sliced_user_buffer_uncrypted: &[u8] = &user_buffer_uncrypted;
    let stdout_string:&str = std::str::from_utf8(&sliced_user_buffer_uncrypted).unwrap();
    stdout_string.to_string()
}

fn chiffrer (msguser_to_send: String) -> Vec<u8>{
    //cypher variables
    //----------------
    type Aes128Cbc = Cbc<Aes128, Pkcs7>;
    let key = hex!("000102030405060708090a0b0c0d0e0f");
    let iv = hex!("f0f1f2f3f4f5f6f7f8f9fafbfcfdfeff");
    let cipher = Aes128Cbc::new_from_slices(&key, &iv).unwrap();
    //---------------
    let msguser_to_send: &[u8] = &msguser_to_send.as_bytes();
    let vec_user_buffer_encrypted = cipher.encrypt_vec(&msguser_to_send);
    vec_user_buffer_encrypted
}

#[tokio::main]
async fn main() {
    let listener = match TcpListener::bind(LISTNER_ADDR).await{
        Err(_) => {
            println!("Adresse déjà utilisé !");
            process::exit(0)
        },
        Ok(listener) => listener,
    };
    println!("En attente d'un client....");

    let (tx, _rx) = broadcast::channel(10);

    loop {
        let (mut socket, addr) = listener.accept().await.unwrap();

        let tx = tx.clone();
        let mut rx = tx.subscribe();
        println!("Nouvelle connexion: {}", addr);

        tokio::spawn(async move {
            let program = Program::new("client Thread".to_string());

            let (reading, mut writing) = socket.split();

            let mut reading = BufReader::new(reading);
            let recved_data: &mut[u8] = &mut [0u8;2048];

            let mut pseudoset = 0;
            let mut pseudo: String = String::new();

            let pseudo_msg = chiffrer(String::from("Entrez votre pseudo :"));
            let pseudo_msg = pseudo_msg.as_slice();

            match writing.write_all(pseudo_msg).await{
                Err(e) => program.print_error(e.to_string()),
                Ok(written) => written,
            };

            loop {
                tokio::select! {
                    resultat_select = reading.read(recved_data) => {

                        //nb d'éléments dans le buffer
                        let buf_size = match resultat_select {
                            Err(erreur) => program.print_fail(erreur.to_string()),
                            Ok(buf_size) => buf_size,
                        };

                        //si on ne reçoit rien alors ça romp le TCPStream
                         if buf_size < 1 {
                            println!("Utilisateur {} s'est déconnecté", addr);
                            break
                         }

                        let new_slice = &recved_data[0..buf_size];
                        let mut recved_string = dechiffrer(&new_slice);

                        if recved_string.ends_with('\n') {
                            recved_string.pop();
                            if recved_string.ends_with('\r') {
                                recved_string.pop();}
                        }

                        if pseudoset != 1 {
                            pseudo = recved_string.clone();
                            println!("L'utilisateur avec l'IP '{}' à choisi le pseudo : {}", addr, pseudo);
                            pseudoset = 1;
                        } else {
                            if recved_string == String::from(":quit"){
                                println!("Utilisateur '{}' avec l'IP {} s'est déconnecté", pseudo, addr);
                                break
                            }

                            println!("{} avec le pseudo '{}' à écrit : {}",addr, pseudo, recved_string);

                            match tx.send((recved_string, addr, pseudo.clone())) {
                                Err(e) => println!("Une erreur est survenue lors de l'envoie du message : {}", e),
                                _ => continue,
                            } //-> rx à besoin d'un &[u8]
                        }
                    }
                    resultat_select = rx.recv() => {
                        let (msguser, other_addresses, pseudo) = match resultat_select {
                            Err(erreur) => program.print_fail(erreur.to_string()),
                            Ok(resultat) => resultat,
                        };

                        if addr != other_addresses {
                            let msguser_to_send = format!("{} a envoyé : {}", pseudo, msguser);

                            let crypted_msg = chiffrer(msguser_to_send.clone());
                            let crypted_msg = crypted_msg.as_slice();

                            //println!("slice avant envoie : {:?}\n", bytes_to_send);

                            match writing.write_all(crypted_msg).await{
                                Err(e) => println!("Une erreur est survenue lors de l'envoie du message : {}", e),
                                _ => continue,
                            };
                        }
                    }
                }
            }
        });
    }
}
