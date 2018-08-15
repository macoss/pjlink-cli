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

extern crate pjlink;

use pjlink::PjlinkDevice;
use std::{env};

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

    let device: PjlinkDevice = if password != "" {
        PjlinkDevice::new_with_password(&host, &password).unwrap()
    } else {
        PjlinkDevice::new(&host).unwrap()
    };

    match device.send_command(&command) {
        Ok(response) => println!("{}", response),
        Err(err) => println!("An error occurred: {}", err),
    }

}