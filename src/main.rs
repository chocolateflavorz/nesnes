use std::fs::File;
use std::io::Read;
use std::io::BufReader;
use clap::Parser;
use nesnes::emu;
#[derive(Parser)]
struct Cli {
    rompath: Option<String>,
}

fn main() {
    if let Some(path) = Cli::parse().rompath {
        let file = File::open(path).unwrap();
        let mut buf_reader = BufReader::new(file);
        let mut contents = Vec::new();
        buf_reader.read_to_end(&mut contents).unwrap();
        emu::run(&contents);
    } else {
        println!("usage: nesnes [ROMFILE]");
    }
}


    