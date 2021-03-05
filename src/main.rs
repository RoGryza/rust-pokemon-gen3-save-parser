mod save;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::SeekFrom;
use std::mem::transmute;

use anyhow::{bail, Result};
use c2rust_bitfields::BitfieldStruct;

use save::SaveSection;

#[repr(C, align(1))]
#[derive(BitfieldStruct)]
struct CompressedPokemon {
    personality: u32,
    otid: u32,
    nickname: [u8; 10],
    language: u8,
    sanity: u8,
    otname: [u8; 7],
    markings: u8,

    // Substructure Growth
    species: u16,
    held_item: u16,

    #[bitfield(name = "pp_bonuses", ty = "u8", bits = "0..=7")]
    #[bitfield(name = "friendship", ty = "u8", bits = "8..=15")]
    #[bitfield(name = "pokeball", ty = "u8", bits = "16..=23")]
    #[bitfield(name = "move1", ty = "u32", bits = "24..=33")]
    #[bitfield(name = "move2", ty = "u32", bits = "34..=43")]
    #[bitfield(name = "move3", ty = "u32", bits = "44..=53")]
    #[bitfield(name = "move4", ty = "u16", bits = "54..=63")]
    growth: [u8; 8],

    //Substructure Condition
    hp_ev: u8,
    atk_ev: u8,
    def_ev: u8,
    spe_ev: u8,
    spa_ev: u8,
    spd_ev: u8,

    //Substructure Misc
    pokerus: u8,
    met_location: u8,
    met_info: u16,
    ivs: u32,
}

fn main() -> Result<()> {
    let mut args = env::args();
    args.next();
    let file = match args.next() {
        Some(filename) => File::open(filename)?,
        None => bail!("Usage: leprogram <file>"),
    };

    for (i, section_res) in SaveSection::read_all(file).enumerate() {
        let section = section_res?;
        println!("Section {} id: {}", i, section.id);
    }

    // let pokemon = read_mon(file)?;
    // println!("Pokemon {:?} {:?}", pokemon.otname, pokemon.nickname);
    Ok(())
}

// fn read_mon<R: Read + Seek>(mut reader: R) -> Result<CompressedPokemon> {
//     reader.seek(SeekFrom::Start(ORIGINAL_BOX_POKEMON_RAM))?;
//     let mut raw_buf = [0u8; 56];
//     reader.read_exact(&mut raw_buf)?;
//     let pokemon = unsafe { transmute(raw_buf) };
//     Ok(pokemon)
// }
