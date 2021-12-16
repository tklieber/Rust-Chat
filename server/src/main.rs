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
use block_modes::{BlockMode, Cbc, block_padding::Pkcs7};
use hex_literal::hex;
use std::str;


//Adresse d'écoute
const LISTNER_ADDR: &str = "127.0.0.1:9999";

// Struct Programme permettant la gestion d'erreur
struct Program {
    name: String
}

impl Program {
    // Attribution du nom du programme instancié
    fn new(name: String) -> Program {
        Program { name }
    }

    // Fonction d'affichage d'une erreur
    fn print_error(&self,mesg: String) {
        println!("{}: Erreur rencontré : {}",self.name ,mesg);
    }

    // Fonction 'fail' qui affiche l'erreur rencontré et arrête le programme
    fn print_fail(&self,mesg: String) -> ! {
        self.print_error(mesg);
        self.fail();
    }

    // Fonction d'arrêt du programme avec le code d'arrêt
    fn exit(&self,status: i32) -> ! { process::exit(status); }
    // Fonction fail qui fait appel à l'arrêt du programme avec le code '-1'
    fn fail(&self) -> ! { self.exit(-1); }
}


fn dechiffrer (recv_data: &[u8]) -> String{
    // cipher variables
    //----------------
    type Aes128Cbc = Cbc<Aes128, Pkcs7>;
    let key = hex!("000102030405060708090a0b0c0d0e0f");
    let iv = hex!("f0f1f2f3f4f5f6f7f8f9fafbfcfdfeff");
    let cipher = Aes128Cbc::new_from_slices(&key, &iv).unwrap();
    //---------------

    // Déchiffre le message reçu et le met dans un Vec<u8>
    let user_buffer_uncrypted = cipher.decrypt_vec(recv_data).unwrap();

    // Le message déchiffré est transformé en String
    //           String <------ Vec<u8>
    let stdout_string = String::from_utf8(user_buffer_uncrypted).unwrap();
    stdout_string
}

fn chiffrer (user_buffer: String) -> Vec<u8>{

    // cipher variables
    //----------------
    type Aes128Cbc = Cbc<Aes128, Pkcs7>;
    let key = hex!("000102030405060708090a0b0c0d0e0f");
    let iv = hex!("f0f1f2f3f4f5f6f7f8f9fafbfcfdfeff");
    let cipher = Aes128Cbc::new_from_slices(&key, &iv).unwrap();
    //---------------

    // Changement de String vers &[u8] pour déchiffrement
    //          &[u8]  <--------  String
    let user_buffer: &[u8] = &user_buffer.as_bytes();

    // Chiffrement du message
    let vec_user_buffer_encrypted = cipher.encrypt_vec(&user_buffer);
    vec_user_buffer_encrypted
}

#[tokio::main]
async fn main() {
    // Attend la connexion d'un client
    let listener = match TcpListener::bind(LISTNER_ADDR).await{
        Err(_) => {
            println!("Adresse déjà utilisé !");
            process::exit(0)
        },
        Ok(listener) => listener,
    };
    println!("En attente d'un client....");

    // Instanciation des Sender et Receiver
    // channel 'Broadcast' permet de diffuser les messages des Sender au Receiver connecté (=abonné)
    let (tx, _rx) = broadcast::channel(10);

    // Loop d'handling des clients connectés
    loop {
        let (mut socket, addr) = listener.accept().await.unwrap();

        // Clone le Sender et l'abonne au Receiver
        let tx = tx.clone();
        let mut rx = tx.subscribe();

        println!("Nouvelle connexion: {}", addr);

        // Nouveau thread d'handling des clients
        // Implémente la struct program
        tokio::spawn(async move {
            let program = Program::new("client Thread".to_string());

            // Split le TcpStream entrant en reader et writer
            let (reader, mut writer) = socket.split();

            // Attribution du buffer au reader
            let mut reader = BufReader::new(reader);
            // Instanciation du buffer
            let recved_data: &mut[u8] = &mut [0u8;2048];

            // Instanciation de l'enregistrement du pseudo envoyé par l'utilisateur
            let mut pseudoset = 0;
            let mut pseudo: String = String::new();

            // Lors de la connexion demande au User d'entrer un pseudo et l'enregistre
            let pseudo_msg = chiffrer(String::from("Entrez votre pseudo :"));
            let pseudo_msg = pseudo_msg.as_slice();
            // Envoie de la demande
            match writer.write_all(pseudo_msg).await{
                Err(e) => program.print_error(e.to_string()),
                Ok(written) => written,
            };

            loop {
                // Feature tokio qui permet d'écouter et d'envoyer des messages en même temps
                tokio::select! {

                    // Écoute du TcpStream entrant
                    resultat_select = reader.read(recved_data) => {
                        //nombre d'éléments dans le buffer
                        let buf_size = match resultat_select {
                            Err(erreur) => program.print_fail(erreur.to_string()),
                            Ok(buf_size) => buf_size,
                        };

                        //si on ne reçoit rien alors ça rompt le TCPStream
                         if buf_size < 1 {
                            if pseudoset == 1 {
                                println!("Utilisateur '{}' avec l'IP {} s'est déconnecté", addr, pseudo);
                                break
                            } else {
                                println!("Utilisateur {} s'est déconnecté", addr);
                            break
                            }
                         }

                        // Instanciation du slice qui fait la taille du message reçu
                        // Necessaire pour le déchiffrement
                        let new_slice = &recved_data[0..buf_size];
                        let mut recved_string = dechiffrer(&new_slice);

                        // Enlève la touche 'entré' du string reçu
                        if recved_string.ends_with('\n') {
                            recved_string.pop();
                            if recved_string.ends_with('\r') {
                                recved_string.pop();}
                        }

                        // Si pas de pseudo -> Définition du pseudo du User
                        // Sinon on continue sans set le pseudo
                        if pseudoset != 1 {
                            pseudo = recved_string.clone();
                            println!("L'utilisateur avec l'IP '{}' à choisi le pseudo : {}", addr, pseudo);
                            pseudoset = 1;
                        }
                        else {
                            // Si l'User entre :quit le TcpStream est rompu
                            if recved_string == String::from(":quit"){
                                println!("Utilisateur '{}' avec l'IP {} s'est déconnecté", pseudo, addr);
                                break
                            }

                            // Print le message reçu par l'User sur le reader du TcpStream
                            println!("{} avec le pseudo '{}' à écrit : {}",addr, pseudo, recved_string);

                            // Envoie à rx pour diffusion du message
                            // Send le message, l'adresse de l'expéditeur et son pseudo
                            match tx.send((recved_string, addr, pseudo.clone())) {
                                // Si erreur lors de l'envoie à rx -> print l'erreur
                                Err(e) => program.print_error(e.to_string()),
                                _ => continue,
                            }
                        }
                    }

                    // Boucle de lecture de rx pour diffusion aux users qui ont leurs tx abonnées
                    resultat_select = rx.recv() => {

                        // Récupère le message, l'adresse de l'expéditeur et son pseudo
                        let (msguser, other_addresses, pseudo) = match resultat_select {
                            Err(erreur) => program.print_fail(erreur.to_string()),
                            Ok(resultat) => resultat,
                        };
                        // Evite de renvoyer le message à l'expediteur
                        if addr != other_addresses {
                            // Transforme le message reçu au bon format
                            let msguser_to_send = format!("{} a envoyé : {}", pseudo, msguser);

                            // Chiffre le message
                            let crypted_msg = chiffrer(msguser_to_send.clone());
                            let crypted_msg: &[u8] = crypted_msg.as_slice();

                            // Envoie du message
                            match writer.write_all(crypted_msg).await{
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
