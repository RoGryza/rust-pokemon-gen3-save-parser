mod pokemon;
mod read;

use std::io::{self, Read, Seek};

pub use pokemon::{PokemonTable, Species};

#[derive(Debug, Clone)]
pub struct Rom {
    pub pokemon: PokemonTable,
}

impl Rom {
    pub fn load<R: Read + Seek>(reader: R) -> io::Result<Self> {
        let pokemon = PokemonTable::load(reader)?;
        Ok(Rom { pokemon })
    }
}
