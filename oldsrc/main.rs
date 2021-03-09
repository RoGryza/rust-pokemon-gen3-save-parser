mod encoding;
mod pokedex;
mod rom;
mod save;
mod types;

pub use types::*;

use std::fs::File;
use std::io::BufReader;

use anyhow::Result;

use pokedex::NationalDexId;
use rom::Rom;
use save::Save;

const ROM_FILE: &str = "rom.bin";
const SAV_FILE: &str = "box.sav";

fn main() -> Result<()> {
    env_logger::init();

    let rom = Rom::load(BufReader::new(File::open(ROM_FILE)?))?;

    for i in 1..=6 {
        let species_id = rom
            .pokemon
            .national_dex_to_species_id(NationalDexId(i))
            .unwrap();
        let poke = rom.pokemon.get_by_species_id(species_id).unwrap();
        println!("Pokemon {}: {}", i, poke.name);
        print!("  {:?}", poke.type1);
        if let Some(t2) = poke.type2 {
            print!("/{:?}", t2);
        }
        println!();
        println!("  catch rate: {}", poke.catch_rate);
        println!("  base EXP yield: {}", poke.exp);
        println!(
            "  EV yields: {}/{}/{}/{}/{}/{}",
            poke.evs[0], poke.evs[1], poke.evs[2], poke.evs[3], poke.evs[4], poke.evs[5]
        );
    }
    println!();

    Ok(())
}
