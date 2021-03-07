mod abilities;
mod moves;
mod pokemon;
mod read;

use std::io::{self, Read, Seek};

pub use abilities::{Ability, AbilityTable};
pub use moves::{Move, MoveTable};
pub use pokemon::{PokemonTable, Species};

#[derive(Debug, Clone)]
pub struct Rom {
    pub abilities: AbilityTable,
    pub moves: MoveTable,
    pub pokemon: PokemonTable,
}

impl Rom {
    pub fn load<R: Read + Seek>(mut reader: R) -> io::Result<Self> {
        let abilities = AbilityTable::load(&mut reader)?;
        let moves = MoveTable::load(&mut reader)?;
        let pokemon = PokemonTable::load(reader)?;
        Ok(Rom {
            abilities,
            moves,
            pokemon,
        })
    }
}
