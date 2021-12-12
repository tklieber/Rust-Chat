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
