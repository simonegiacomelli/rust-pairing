use std::fs::{File, OpenOptions};

fn main() {
    let file = OpenOptions::new().append(true).create(true).open("ciccio.txt");
    match file {
        Ok(f) => { println!("Ok") }
        Err(e) => { println!("Err, {e}") }
    }
}