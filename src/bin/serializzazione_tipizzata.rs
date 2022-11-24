use std::fs;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};

use serde::{Deserialize, Serialize};

fn main() {
    println!("Hello, world!");

    let data = "Some data!";
    let mut f = OpenOptions::new()
        .write(true)
        .append(true)
        .open("foo.txt")
        .unwrap();

    f.write_all(b"pippo\n").unwrap();
    f.sync_all().unwrap();

    let mut r = OpenOptions::new()
        .read(true)
        .open("foo.txt")
        .unwrap();
    let mut str = String::new();
    let size = r.read_to_string(&mut str);

    #[derive(Debug, Serialize, Deserialize)]
    struct Pippo {
        name: String,
        age: i8,
    }

    impl Pippo {
        fn debug() -> String {
            String::from("2")
        }
    }
    let pippo = Pippo { name: String::from("ciao"), age: 8 };
    println!("{} {}", pippo.name, pippo.age);
    println!("{:?}", pippo);
}