use std::collections::{HashMap, HashSet};
use std::io::{self, Read, Seek};

use super::national_dex::{IGNORED_NATIONAL_DEX, NATIONAL_DEX, SHORTENED_NAMES};
use super::read::RomReadExt;
use crate::pokedex::{NationalDexId, SpeciesId};

const NUM_SPECIES: usize = 1268;
const POKEMON_NAME_LENGTH: usize = 10;
const SPECIES_NAMES_OFFSET: u64 = 0x0144;

#[derive(Debug, Clone)]
pub struct PokemonTable {
    species: Vec<Species>,
    species_to_national_dex_id: HashMap<SpeciesId, NationalDexId>,
    national_dex_to_species_id: HashMap<NationalDexId, SpeciesId>,
}

#[derive(Debug, Clone)]
pub struct Species {
    pub name: String,
}

impl PokemonTable {
    pub fn load<R: Read + Seek>(mut reader: R) -> io::Result<Self> {
        let ignored_national_dex: HashSet<_> = IGNORED_NATIONAL_DEX.iter().cloned().collect();
        let shortened_to_orig_name: HashMap<_, _> = SHORTENED_NAMES.iter().cloned().collect();

        reader.seek_pointer_at(SPECIES_NAMES_OFFSET)?;
        let mut species = Vec::new();
        let mut name_to_species_id = HashMap::new();
        for (i, name_res) in reader
            .read_string_list(NUM_SPECIES, POKEMON_NAME_LENGTH + 1)
            .enumerate()
        {
            let name = name_res?;
            if !ignored_national_dex.contains(name.as_str()) {
                let orig_name = match shortened_to_orig_name.get(name.as_str()) {
                    Some(orig) => orig.to_string(),
                    None => name.clone(),
                };
                name_to_species_id.insert(orig_name, SpeciesId(i as u16));
            }
            species.push(Species { name });
        }

        let mut species_to_national_dex_id = HashMap::new();
        let mut national_dex_to_species_id = HashMap::new();
        for (i, name) in NATIONAL_DEX.iter().enumerate() {
            match name_to_species_id.remove(*name) {
                Some(species_id) => {
                    let national_dex_id = NationalDexId((i + 1) as u16);
                    species_to_national_dex_id.insert(species_id, national_dex_id);
                    national_dex_to_species_id.insert(national_dex_id, species_id);
                }
                None => log::warn!("Species ID for name {:?} not found", name),
            }
        }

        for name in name_to_species_id.keys() {
            log::warn!("National dex ID for name {:?} not found", name);
        }

        Ok(PokemonTable {
            species,
            species_to_national_dex_id,
            national_dex_to_species_id,
        })
    }

    pub fn get_by_species_id(&self, species_id: SpeciesId) -> Option<&Species> {
        self.species.get(species_id.0 as usize)
    }

    pub fn national_dex_to_species_id(&self, national_dex_id: NationalDexId) -> Option<SpeciesId> {
        self.national_dex_to_species_id
            .get(&national_dex_id)
            .cloned()
    }

    pub fn species_to_natinal_dex_id(&self, species_id: SpeciesId) -> Option<NationalDexId> {
        self.species_to_national_dex_id.get(&species_id).cloned()
    }
}
