use std::io::{self, Read, Seek};

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
        for name_res in reader.read_string_list(NUM_SPECIES, POKEMON_NAME_LENGTH + 1) {
            let name = name_res?;
            species.push(Species { name });
        }
        Ok(PokemonTable { species })
    }

    pub fn get_by_species_id(&self, species_id: u16) -> Option<&Species> {
        self.species.get(species_id as usize)
    }
}
