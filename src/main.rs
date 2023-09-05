use clap::Parser;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;

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
    } else {
        println!("usage: nesnes [ROMFILE]");
    }
}

#[test]
fn test() {
    use nesnes::emu::Emu;
    let mut emu = Emu::default();
    emu.load(vec![0x00, 0x00, 0x00, 0x00]);
    emu.run();
}
