// Copyright 2018 Rick Russell
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at

// http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.


use std::{env, str};
use std::io::prelude::*;
use std::net::TcpStream;
extern crate md5;

const AUTH: char = '1';
const NOAUTH: char = '0';

fn send_command(host: &str, cmd: &str, pwd: &str) -> Result<String, std::io::Error> {
	let host_port = [host, ":4352"].concat();
    let mut client_buffer = [0u8; 256];
    let mut stream = try!(TcpStream::connect(host_port));
    let mut command = String::from("");

	let _ = stream.read(&mut client_buffer); //Did we get the hello string?

	match client_buffer[7] as char { // Does the connection require auth or not
        AUTH => { // Connection requires auth
            let rnd_num = String::from_utf8_lossy(&client_buffer[9..17]).to_string();
            let pwd_str = format!("{}{}", rnd_num, pwd);
            let digest = md5::compute(pwd_str);
            command = format!("{:x}%1{}\r", digest, cmd);
        },
        NOAUTH => { // Connection requires no auth
            command = format!("%1{}\r", cmd);
        },

        _ => println!("Unable to deturim if auth if needed."),
    }
    
    let result = stream.write(command.as_bytes());
    match result {
        Ok(_) => (),
        Err(e) => return Err(e),

    };
    let result = stream.read(&mut client_buffer);
    let len = match result {
        Ok(len) => len,
        Err(e) => return Err(e), 
    };

    let response = String::from_utf8_lossy(&client_buffer[0..len]).to_string();
    Ok(response)
}

fn main() {
    let host = match env::args().nth(1) {
        Some(hst) => hst,
        None => {
            let my_name = env::args().nth(0).unwrap();
            panic!("Usage: {} [host] [command] [password]", my_name)
        }
    };

    let command = match env::args().nth(2) {
        Some(cmd) => cmd,
        None => {
            let my_name = env::args().nth(0).unwrap();
            panic!("Usage: {} [host] [command] [password]", my_name)
        }
    };

    let password = match env::args().nth(3) {
        Some(pwd) => pwd,
        None => String::from("")
    };

    match send_command(&host, &command, &password) {
        Ok(response) => println!("{}", response),
        Err(err) => println!("An error occurred: {}", err),
    }
}