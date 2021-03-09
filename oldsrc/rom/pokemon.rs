use std::collections::{HashMap, HashSet};
use std::io::{self, Read, Seek, SeekFrom};

use byteorder::{LittleEndian, ReadBytesExt};

use super::abilities::AbilityId;
use super::items::ItemId;
use super::national_dex::{IGNORED_NATIONAL_DEX, NATIONAL_DEX, SHORTENED_NAMES};
use super::read::RomReadExt;
use crate::pokedex::{NationalDexId, SpeciesId};

const NUM_SPECIES: usize = 1268;
const POKEMON_NAME_LENGTH: usize = 10;
const SPECIES_NAMES_OFFSET: u64 = 0x0144;
const BASE_STATS_OFFSET: u64 = 0x01BC;

#[derive(Debug, Clone)]
pub struct PokemonTable {
    species: Vec<Species>,
    species_to_national_dex_id: HashMap<SpeciesId, NationalDexId>,
    national_dex_to_species_id: HashMap<NationalDexId, SpeciesId>,
}

#[derive(Clone, Copy, Debug)]
pub enum PokemonType {
    Normal,
    Fighting,
    Flying,
    Poison,
    Ground,
    Rock,
    Bug,
    Ghost,
    Steel,
    Fire,
    Water,
    Grass,
    Electric,
    Psychic,
    Ice,
    Dragon,
    Dark,
    Fairy,
    Unknown(u8),
}

#[derive(Debug, Clone)]
pub struct Species {
    pub name: String,

    pub base_stats: [u8; 6],
    pub type1: PokemonType,
    pub type2: Option<PokemonType>,

    pub catch_rate: u8,
    pub exp: u8,
    pub evs: [u8; 6],

    pub hold_item1: Option<ItemId>,
    pub hold_item2: Option<ItemId>,

    pub gender_ratio: u8,
    pub egg_cycles: u8,
    pub base_friendship: u8,
    pub growth_rate: u8,
    pub egg_group1: u8,
    pub egg_group2: u8,

    pub ability1: AbilityId,
    pub ability2: Option<AbilityId>,
    pub hidden_ability: Option<AbilityId>,
}

impl PokemonTable {
    pub fn load<R: Read + Seek>(mut reader: R) -> io::Result<Self> {
        let ignored_national_dex: HashSet<_> = IGNORED_NATIONAL_DEX.iter().cloned().collect();
        let shortened_to_orig_name: HashMap<_, _> = SHORTENED_NAMES.iter().cloned().collect();

        let mut species = Vec::new();
        reader.seek_pointer_at(BASE_STATS_OFFSET)?;
        for _ in 0..NUM_SPECIES {
            let st = Species::read_base_stats(&mut reader)?;
            species.push(st);
        }

        reader.seek_pointer_at(SPECIES_NAMES_OFFSET)?;
        let mut name_to_species_id = HashMap::new();
        for (i, (name_res, current_species)) in reader
            .read_string_list(NUM_SPECIES, POKEMON_NAME_LENGTH + 1)
            .zip(&mut species)
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
            current_species.name = name;
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

impl PokemonType {
    pub fn parse(raw: u8) -> Self {
        match raw {
            0x00 => PokemonType::Normal,
            0x01 => PokemonType::Fighting,
            0x02 => PokemonType::Flying,
            0x03 => PokemonType::Poison,
            0x04 => PokemonType::Ground,
            0x05 => PokemonType::Rock,
            0x06 => PokemonType::Bug,
            0x07 => PokemonType::Ghost,
            0x08 => PokemonType::Steel,
            0x0a => PokemonType::Fire,
            0x0b => PokemonType::Water,
            0x0c => PokemonType::Grass,
            0x0d => PokemonType::Electric,
            0x0e => PokemonType::Psychic,
            0x0f => PokemonType::Ice,
            0x10 => PokemonType::Dragon,
            0x11 => PokemonType::Dark,
            0x17 => PokemonType::Fairy,
            x => PokemonType::Unknown(x),
        }
    }
}

impl Species {
    pub fn read_base_stats<R: Read + Seek>(mut reader: R) -> io::Result<Self> {
        let mut base_stats = [0u8; 6];
        reader.read_exact(&mut base_stats)?;
        base_stats.swap(3, 5);
        base_stats.swap(4, 5);

        let mut raw_types = [0u8; 2];
        reader.read_exact(&mut raw_types)?;
        let type1 = PokemonType::parse(raw_types[0]);
        let type2 = PokemonType::parse(raw_types[1]);

        let catch_rate = reader.read_u8()?;
        let exp = reader.read_u8()?;

        let raw_evs = reader.read_u16::<LittleEndian>()?;
        let mut evs = [0; 6];
        evs[0] = ((raw_evs >> 0) & 0b11) as u8;
        evs[1] = ((raw_evs >> 2) & 0b11) as u8;
        evs[2] = ((raw_evs >> 4) & 0b11) as u8;
        evs[5] = ((raw_evs >> 6) & 0b11) as u8;
        evs[3] = ((raw_evs >> 8) & 0b11) as u8;
        evs[4] = ((raw_evs >> 10) & 0b11) as u8;

        let raw_hold_item1 = reader.read_u16::<LittleEndian>()?;
        let raw_hold_item2 = reader.read_u16::<LittleEndian>()?;

        let gender_ratio = reader.read_u8()?;
        let egg_cycles = reader.read_u8()?;
        let base_friendship = reader.read_u8()?;
        let growth_rate = reader.read_u8()?;
        let egg_group1 = reader.read_u8()?;
        let egg_group2 = reader.read_u8()?;

        let ability1 = AbilityId(reader.read_u8()?);
        let raw_ability2 = reader.read_u8()?;
        reader.seek(SeekFrom::Current(2))?; // Skip safariZoneFleeRate and bodyColor/noFlip
        let raw_hidden_ability = reader.read_u8()?;

        Ok(Species {
            name: String::new(),

            base_stats,
            type1,
            type2: Some(type2),

            catch_rate,
            exp,
            evs,

            hold_item1: if raw_hold_item1 == 0 {
                None
            } else {
                Some(ItemId(raw_hold_item1))
            },
            hold_item2: if raw_hold_item2 == 0 {
                None
            } else {
                Some(ItemId(raw_hold_item2))
            },

            gender_ratio,
            egg_cycles,
            base_friendship,
            growth_rate,
            egg_group1,
            egg_group2,

            ability1,
            ability2: if raw_ability2 == 0 {
                None
            } else {
                Some(AbilityId(raw_ability2))
            },
            hidden_ability: if raw_hidden_ability == 0 {
                None
            } else {
                Some(AbilityId(raw_hidden_ability))
            },
        })
    }
}
