//
// Rust : Projet de messagerie instantanée
//
// Auteurs : Quentin CHARLES, Nicolas TAHON, Tristan KLIEBER
//
// Programme client

use std::string::String;
use std::process;
use std::thread;
use std::io::{self, Read, Write};
use std::net::TcpStream;
use aes::Aes128;
use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;
use hex_literal::hex;



const SERVER_ADDR:&str = "127.0.0.1:9999";


//fonction de déchiffrement du message reçu
fn dechiffrer (client_buffer: &[u8]) -> String{
    // cipher variables
    //----------------
    type Aes128Cbc = Cbc<Aes128, Pkcs7>;
    let key = hex!("000102030405060708090a0b0c0d0e0f");
    let iv = hex!("f0f1f2f3f4f5f6f7f8f9fafbfcfdfeff");
    let cipher = Aes128Cbc::new_from_slices(&key, &iv).unwrap();
    //---------------

    // Déchiffre le message reçu et le met dans un Vec<u8>
    let user_buffer_uncrypted = cipher.decrypt_vec(client_buffer).unwrap();

    // Le message déchiffré est transformé en String
    //           String <------ Vec<u8>
    let stdout_string = String::from_utf8(user_buffer_uncrypted).unwrap();
    stdout_string
}


//fonction de chiffrement du message à envoyer
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


fn main() {
    // Création de la Struct 'Programme' avec le nom 'Programme Client'
    let program = Program::new("Programme Client".to_string());

    // Tentative de connexion au serveur
    // Si la connexion échoue 'print fail' de la Struct Programme est appelé : affiche le message d'erreur et arrête le programme
    let mut stream = TcpStream::connect(SERVER_ADDR)
        .unwrap_or_else(|error|
            program.print_fail(error.to_string()));

    // Clone le TcpStream
    // Si ça échoue 'print fail' est appelé : affiche le message d'erreur et arrête le programme
    let mut input_stream = stream.try_clone()
        .unwrap_or_else(|error|
            program.print_fail(error.to_string()));

    // Thread de réception des messages avec move des valeurs
    // Thread nécessaire car nous avons besoin d'écouter le flux entrant
    // et en même temps lire l'entrée utilisateur (stdin)
    thread::spawn(move || {
        // Instanciation du buffer de lecture du flux entrant
        let mut client_buffer: &mut [u8] = &mut [0u8;2048];

        //Boucle de lecture du TcpStream
        loop {
            match input_stream.read(&mut client_buffer) {
                Ok(n) => {
                    // Si on reçoit plus rien sur le TcpStream
                    // --> print du message d'erreur et arrêt du programme
                    //     avec 'exit' de la Struct 'Programme'
                    if n == 0 {
                        println!("Connexion avec le serveur interrompu !");
                        program.exit(1);
                    }
                    else
                    {
                        // Définition d'une slice ayant la taille du message reçu
                        let new_slice = &client_buffer[0..n];

                        // Déchiffrement du message reçu
                        let printed_strings = dechiffrer(&new_slice);
                        println!("{}",printed_strings);
                    }
                },
                // Si le il y a eu une erreur de lecture du TcpStream entrant
                // --> print l'erreur et arrête le programme
                Err(error) => program.print_fail(error.to_string()),
            }
        }
    });

    // Instanciation du buffer d'envoie
    let mut user_buffer = String::new();

    // Loop de lecteur de l'entré utilisateur et d'envoie de cette entrée
    loop {
        match io::stdin().read_line(&mut user_buffer){
            Ok(_) => {
                //Rien à faire si ça marche :)
            },
            // Print de l'erreur rencontré lors de
            Err(e) => println!("Une erreur est survenue : {}", e)
        }

        // Chriffrement du message avant envoie
        let user_buffer_encrypted = chiffrer(user_buffer.clone());
        // Besoin de transformé le Vec en slice pour envoie
        //          &[u8]  <------  Vec<u8>
        let final_sent_buffer:&[u8] = &user_buffer_encrypted;

        // Envoie du message sur le TcpStream
        stream.write_all(final_sent_buffer).unwrap();

        // Clear du buffer stdin sinon les messages envoyés précédemments restent enregistrés
        user_buffer.clear();
    }
}
