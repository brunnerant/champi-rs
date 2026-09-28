use std::{env::args, fs};

use champis::champi::Champi;

fn main() {
    let file = args().nth(1).unwrap();
    let champis: Vec<Champi> = serde_json::from_reader(fs::File::open(file).unwrap()).unwrap();
    for champi in &champis {
        println!("{:?}", champi);
    }
}
