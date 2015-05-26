#[macro_use]
extern crate qmlrs;
extern crate self_encryption;
extern crate rand;
//extern crate tempdir;
extern crate docopt;
extern crate rustc_serialize;
extern crate cbor;

use std::fmt;
// use std::fs;
use std::fs::{File};
use std::io::prelude::*;
use std::path::Path;
use std::string::String;
// use std::error::Error;
use std::sync::Arc;

// use docopt::Docopt;
use cbor::{ Encoder};
// use cbor::{ Encoder, Decoder};

use self_encryption::*;

struct SelfEncrypt;
impl SelfEncrypt {
    fn encrypt(&self, filepath: String) -> String {
        let my_storage = Arc::new(MyStorage { storage_path : "chunk_store_test/".to_string() });
        let mut file = match File::open(&filepath.clone()) {
              Err(_) => panic!("couldn't open {}", filepath.clone()),
              Ok(f) => f,
            };
        let mut data = Vec::new();
        file.read_to_end(&mut data).unwrap();

        let mut se = SelfEncryptor::new(my_storage.clone(), datamap::DataMap::None);
        se.write(&data, 0);
        let data_map = se.close();
        let mut file = match File::create("data_map") {
            Err(_) => panic!("couldn't create data_map"),
            Ok(f) => f
        };
        let mut encoded = Encoder::from_memory();
        encoded.encode(&[&data_map]).unwrap();
        match file.write_all(&encoded.as_bytes()[..]) {
            Err(_) => panic!("couldn't write "),
            Ok(_) => println!("chunk  written")
        };
        let done = "Done".to_string();
        done
    }
}

Q_OBJECT! { SelfEncrypt:
    slot fn encrypt(String);
}

fn to_hex(ch: u8) -> String {
    let hex = fmt::format(format_args!("{:x}", ch));
    if hex.len() == 1 {
        let s = "0".to_string();
        s + &hex
    } else {
        hex
    }
}

fn file_name(name: &Vec<u8>) -> String {
    let mut string = String::new();
    for i in 0..name.len() {
        string.push_str(&to_hex(name[i]));
    }
    string
}

pub struct MyStorage {
    pub storage_path : String
}

impl Storage for MyStorage {
  fn get(&self, name: Vec<u8>) -> Vec<u8> {
    let pathstr = file_name(&name);
    let tmpname = self.storage_path.clone() + &pathstr;
    let path = Path::new(&tmpname);
    let display = path.display();
    let mut file = match File::open(&path) {
      Err(_) => panic!("couldn't open {}", display),
        Ok(f) => f,
    };
    let mut data = Vec::new();
    file.read_to_end(&mut data).unwrap();
    data
  }

  fn put(&self, name: Vec<u8>, data: Vec<u8>) {
    let pathstr = file_name(&name);
    let tmpname = self.storage_path.clone() + &pathstr;
    let path = Path::new(&tmpname);
    let mut file = match File::create(&path) {
           Err(_) => panic!("couldn't create"),
           Ok(f) => f
    };

    match file.write_all(&data[..]) {
             Err(_) => panic!("couldn't write "),
             Ok(_) => println!("chunk  written")
    };
  }
}


fn main() {
    let mut engine = qmlrs::Engine::new();

    engine.set_property("SelfEncrypt", SelfEncrypt);
    engine.load_local_file("examples/self_encryptor_ui.qml");

    engine.exec();
}
