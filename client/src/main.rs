#![allow(unused_variables)]
#![allow(unused_imports)]

use openssl::rsa::{Padding, Rsa};
use openssl::symm::Cipher;
use std::env;
use std::str::from_utf8;
use std::process;
use std::thread;
use std::io::{self, Read, Write, Error};
use std::net::TcpStream;
use std::net::TcpListener;


const SERVER_ADDR:&str = "127.0.0.1:9999";


struct Program {
    name: String
}
/*
fn encrypt (user_buffer: &mut String, cipher: Cbc<Aes128, Pkcs7>) -> &[u8] {
    let user_buffer_encrypted = cipher.encrypt_vec(user_buffer as [u8]);
    user_buffer_encrypted
}
fn decrypt (user_buffer_encrypted : &mut String, cipher: Cbc<Aes128, Pkcs7>) -> &[u8] {
    let user_buffer_decrypted = cipher.decrypt_vec(&user_buffer_encrypted).unwrap();
    user_buffer_decrypted
}
 */


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

        /*
        -----------------
        CIPHER TEST
        -----------------
        type Aes128Cbc = Cbc<Aes128, Pkcs7>;
        let key = hex!("000102030405060708090a0b0c0d0e0f");
        let iv = hex!("f0f1f2f3f4f5f6f7f8f9fafbfcfdfeff");
        let cipher = Aes128Cbc::new_from_slices(&key, &iv).unwrap();
        let user_buffer_encrypted = cipher.encrypt_vec(user_buffer.as_bytes());
        //let user_buffer_encrypted: String = encrypt(&mut user_buffer, cipher);
        //let to_stringed_user_buffer = std::str::from_utf8(&user_buffer_encrypted).unwrap();
        //println!("{:?} -> message chiffré", to_stringed_user_buffer);
        */
        let rsa = Rsa::generate(512).unwrap();
        let mut cryptage = vec![0; rsa.size() as usize];
        let _ = rsa.public_encrypt(user_buffer.as_bytes(), &mut cryptage, Padding::PKCS1).unwrap();
        println!("Cryptage caractères : {}", String::from_utf8_lossy(cryptage.as_slice()));
        let buffer_to_send = String::from_utf8_lossy(cryptage.as_slice());

        output_stream.write_all(buffer_to_send.as_bytes()).unwrap();
        output_stream.flush().unwrap();
        user_buffer.clear();
    }
}
