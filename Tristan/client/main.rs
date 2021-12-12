#![allow(unused_variables)]
#![allow(unused_imports)]

use openssl::base64::{decode_block, encode_block};
use std::env;
use std::process;
use std::thread;
use std::io::{self, Read, Write, Error};
use std::net::TcpStream;
use std::net::TcpListener;


const SERVER_ADDR:&str = "127.0.0.1:9999";


struct Program {
    name: String
}

fn encrypt (user_buffer: &mut String) -> String {
    let user_buffer_encrypted = encode_block(user_buffer.as_bytes() as &[u8]);
    user_buffer_encrypted
}

impl Program {
    fn new(name: String) -> Program {
        Program { name: name }
    }

    fn print_error(&self,mesg: String) {
        println!("{}: error: {}",self.name,mesg);
    }

    fn print_fail(&self,mesg: String) -> ! {
        self.print_error(mesg);
        self.fail();
    }

    fn exit(&self,status: i32) -> ! { process::exit(status); }
    fn fail(&self) -> ! { self.exit(-1); }
}


fn main() {
    let mut args = env::args();
    let program = Program::new(
        args.next().unwrap_or("test".to_string())
    );

    let mut stream = TcpStream::connect(SERVER_ADDR).unwrap_or_else(|error|
        program.print_fail(error.to_string())
    );
    let mut input_stream = stream.try_clone().unwrap();

    let handler = thread::spawn(move || {
        let mut client_buffer = vec![0u8];

        loop {
            match input_stream.read(&mut client_buffer) {
                Ok(n) => {
                    if n == 0 {
                        println!("Connexion avec le serveur interrompu !");
                        program.exit(0);
                    }
                    else
                    {
                        io::stdout().write_all(&client_buffer).unwrap();
                        io::stdout().flush().unwrap();
                    }
                },
                Err(error) => program.print_fail(error.to_string()),
            }
        }
    });

    let output_stream = &mut stream;
    let mut user_buffer = String::new();

    loop {

        io::stdin().read_line(&mut user_buffer).unwrap();

        let user_buffer_encrypted: String = encrypt(&mut user_buffer);
        println!("{} -> message chiffré", user_buffer_encrypted);

        output_stream.write_all(user_buffer_encrypted.as_bytes()).unwrap();
        output_stream.flush().unwrap();
        user_buffer.clear();
    }
}

/*
use std::sync::thread;
use std::io;
use tokio::{
    net::TcpStream,
    io::{AsyncWriteExt,AsyncBufReadExt, BufReader},
};

const SERVER_ADDR: &str = "127.0.0.1:9999";


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
            println!("> entré : {}", buffer);
            return buffer
        },
        Err(e) => println!("Une erreur est survenue : {}", e)
    }
    buffer
}


/*
//a adapter
fn send_and_receive_msg(mut stream: &TcpStream){
    loop{
        let input: String = user_input();
        let msg = input.as_bytes();
        let msglen = msg.len();

        stream.write(msg);
        print!("Message envoyé, en attente de la réponse.....");

        //let mut data = [0 as u8; 25]; // buffer 6 bytes
        let mut data: Vec<u8> = vec![0; msglen];

        /*
        PLUS NÉCESSAIRE -> UN SIMPLE READ SUFFIT
        match stream.read_exact(&mut data) {
            // A MODIFIER !!! -> ne renvoie pas la rep
            Ok(_) => {
                if &data == msg {
                    println!("Message reçu!");
                } else {
                    let text = from_utf8(&data).unwrap();
                    println!("Réponse unexpected: {}", text);
                }
            },
            Err(e) => {
                println!("Erreur dans la réception des données: {}", e);
            }
        }
        */
    }
}
*/
#[tokio::main]
async fn main() {
    let mut connection = TcpStream::connect(SERVER_ADDR).await.unwrap();
    println!("connected");

    let (reader, mut writer) =  connection.split();
    let mut reader = BufReader::new(reader);


    //USELESS
    let mut addr = &connection.local_addr().unwrap();
    println!("{}",addr);

    let mut recved_data = String::new();

    tokio::spawn(async move {
        loop{
            let input: String = user_input();

                    thread::select! {
                        //lit la réception de TCPListener et l'affiche = affiche les msg entrant
                        resultat_select = reader.read_line(&mut recved_data) => {
                            //si on reçoit rien -> break
                            if resultat_select.unwrap() == 0 { return }

                            println!("your address{}",addr);
                            //println!("{}", resultat_select);
                            recved_data.clear();
                        }
                        //Lit et envoie stdin = affiche et envoie le stdin
                        resultat_select = writer.write_all(input.as_bytes()).await.unwrap() => {
                            match resultat_select {
                                Ok(_) => { println!("send") },
                                Err(e) => {println!("Une Erreur est survenue : {}", e)}
                            }
                            println!("ok");
                        }
                    }
            }
        });
}

/*
----------------------------------------------
                OLD
----------------------------------------------
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
    match TcpStream::connect("localhost:9999") {
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
*/
*/
