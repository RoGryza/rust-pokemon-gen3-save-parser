use std::io::{self, Read, Seek};

use crate::encoding::parse_string_lossy;

use super::read::RomReadExt;

const NUM_SPECIES: usize = 1268;
const POKEMON_NAME_LENGTH: usize = 10;
const SPECIES_NAMES_OFFSET: u64 = 0x0144;

#[derive(Debug, Clone)]
pub struct PokemonTable {
    species: Vec<Species>,
}

#[derive(Debug, Clone)]
pub struct Species {
    pub name: String,
}

impl PokemonTable {
    pub fn load<R: Read + Seek>(mut reader: R) -> io::Result<Self> {
        reader.seek_pointer_at(SPECIES_NAMES_OFFSET)?;
        let mut species = Vec::new();
        for _ in 0..NUM_SPECIES {
            let mut raw_name = [0u8; POKEMON_NAME_LENGTH + 1];
            reader.read_exact(&mut raw_name)?;
            let name = parse_string_lossy(&raw_name);
            species.push(Species { name });
        }
        Ok(PokemonTable { species })
    }

    pub fn get_by_species_id(&self, species_id: u16) -> Option<&Species> {
        self.species.get(species_id as usize)
    }
}
