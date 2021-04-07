use std::fs::File;
use std::io::BufReader;

use poke3_radicalred::data::read_rom;

fn main() {
    let reader = BufReader::new(File::open("rom.bin").unwrap());
    let rom = read_rom(reader).unwrap();
    println!("{}", serde_json::to_string_pretty(&rom).unwrap());
}
