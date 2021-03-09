use std::io::{self, Read, Seek};

use super::read::RomReadExt;

const NUM_ABILITIES: usize = 0xFE + 1;
const ABILITY_NAME_LENGTH: usize = 16;
const ABILITY_NAMES_OFFSET: u64 = 0x0001C0;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct AbilityId(pub u8);

#[derive(Debug, Clone)]
pub struct AbilityTable {
    abilities: Vec<Ability>,
}

#[derive(Debug, Clone)]
pub struct Ability {
    pub name: String,
}

impl AbilityTable {
    pub fn load<R: Read + Seek>(mut reader: R) -> io::Result<Self> {
        reader.seek_pointer_at(ABILITY_NAMES_OFFSET)?;
        let mut abilities = Vec::new();
        for name_res in reader.read_string_list(NUM_ABILITIES, ABILITY_NAME_LENGTH + 1) {
            let name = name_res?;
            abilities.push(Ability { name });
        }
        Ok(AbilityTable { abilities })
    }

    pub fn get_by_ability_id(&self, ability_id: AbilityId) -> Option<&Ability> {
        self.abilities.get(ability_id.0 as usize)
    }
}
