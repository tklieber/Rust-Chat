#![allow(unused_variables)]

use std::process;
use std::thread;
use std::io::{self, Read, Write};
use std::net::TcpStream;
use aes::Aes128;
use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;
use hex_literal::hex;


const SERVER_ADDR:&str = "127.0.0.1:9999";


fn dechiffrer (client_buffer: &[u8]) -> String{
    //cypher variables
    //----------------
    type Aes128Cbc = Cbc<Aes128, Pkcs7>;
    let key = hex!("000102030405060708090a0b0c0d0e0f");
    let iv = hex!("f0f1f2f3f4f5f6f7f8f9fafbfcfdfeff");
    let cipher = Aes128Cbc::new_from_slices(&key, &iv).unwrap();
    //---------------
    let user_buffer_uncrypted = cipher.decrypt_vec(client_buffer).unwrap();
    let prefinal_buffer: &[u8] = &user_buffer_uncrypted;
    let stdout_string:&str = std::str::from_utf8(&prefinal_buffer).unwrap();
    stdout_string.to_string()
}

struct Program {
    name: String
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
    let program = Program::new("my programme".to_string());

    let mut stream = TcpStream::connect(SERVER_ADDR).unwrap_or_else(|error|
        program.print_fail(error.to_string())
    );
    let mut input_stream = stream.try_clone().unwrap();

    let handler = thread::spawn(move || {
        let mut client_buffer: &mut [u8] = &mut [0u8;2048];

        loop {
            match input_stream.read(&mut client_buffer) {
                Ok(n) => {
                    if n == 0 {
                        println!("Connexion avec le serveur interrompu !");
                        program.exit(0);
                    }
                    else
                    {
                        let new_slice = &client_buffer[0..n];
                        let printed_strings = dechiffrer(&new_slice);
                        println!("{}",printed_strings);
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
        */
        type Aes128Cbc = Cbc<Aes128, Pkcs7>;
        let key = hex!("000102030405060708090a0b0c0d0e0f");
        let iv = hex!("f0f1f2f3f4f5f6f7f8f9fafbfcfdfeff");
        let cipher = Aes128Cbc::new_from_slices(&key, &iv).unwrap();
        //encrypt the data
        let user_buffer_encrypted = cipher.encrypt_vec(user_buffer.as_bytes());
        //  Vec<u8>  ->  &[u8]
        let final_sent_buffer:&[u8] = &user_buffer_encrypted;
        //-----------> on envoie un &[u8]
        output_stream.write_all(final_sent_buffer).unwrap();
        output_stream.flush().unwrap();
        user_buffer.clear();
    }
}
