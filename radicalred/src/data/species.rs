use std::convert::TryFrom;

use bitfield::bitfield;

use poke3_common::encoding::parse_string;
use poke3_common::rom::{AbilityId, EggGroup, ItemId, PokemonType, Species, Stats};

use super::ValidationError;
use crate::read::{FromTable, Named};
use crate::shortened_names::expand_name;

pub type RawPokemon = Named<RawPokemonName, RawBaseStats>;

#[repr(transparent)]
pub struct RawPokemonName([u8; 11]);

#[repr(C, align(1))]
pub struct RawBaseStats {
    base_hp: u8,
    base_atk: u8,
    base_def: u8,
    base_spe: u8,
    base_spa: u8,
    base_spd: u8,
    type1: u8,
    type2: u8,
    catch_rate: u8,
    exp: u8,
    evs: EvYields,
    item1: u16,
    item2: u16,
    gender_ratio: u8,
    egg_cycles: u8,
    friendship: u8,
    growth_rate: u8,
    egg_group1: u8,
    egg_group2: u8,
    ability1: u8,
    ability2: u8,
    _safari_zone_flee_rate: u8,
    _ignore_body_color_no_flip: u8,
    hidden_ability: u8,
}

bitfield! {
    struct EvYields(u16);
    u8;
    #[inline] pub hp, _: 1, 0;
    #[inline] pub atk, _: 3, 2;
    #[inline] pub def, _: 5, 4;
    #[inline] pub spe, _: 7, 6;
    #[inline] pub spa, _: 9, 8;
    #[inline] pub spd, _: 11, 10;
}

impl FromTable for RawBaseStats {
    const NAME: &'static str = "gBaseStats";
    const COUNT: usize = 1268;
    const OFFSET: u64 = 0x01BC;
}

impl FromTable for RawPokemonName {
    const NAME: &'static str = "gSpeciesNames";
    const COUNT: usize = RawBaseStats::COUNT;
    const OFFSET: u64 = 0x0144;
}

impl TryFrom<RawPokemon> for Species {
    type Error = ValidationError;

    fn try_from(raw: RawPokemon) -> Result<Species, ValidationError> {
        Ok(Species {
            original_name: expand_name(&raw.name.0).map(|s| s.to_string()),
            ingame_name: parse_string(&raw.name.0)
                .map_err(|e| ValidationError::from_display("name", e))?,
            base_stats: Stats {
                hp: raw.value.base_hp,
                atk: raw.value.base_atk,
                def: raw.value.base_def,
                spa: raw.value.base_spa,
                spd: raw.value.base_spd,
                spe: raw.value.base_spe,
            },
            type1: map_pokemon_type(raw.value.type1),
            type2: if raw.value.type2 == raw.value.type1 {
                None
            } else {
                Some(map_pokemon_type(raw.value.type2))
            },
            catch_rate: raw.value.catch_rate,
            exp: raw.value.exp,
            ev_yield: Stats {
                hp: raw.value.evs.hp(),
                atk: raw.value.evs.atk(),
                def: raw.value.evs.def(),
                spa: raw.value.evs.spa(),
                spd: raw.value.evs.spd(),
                spe: raw.value.evs.spe(),
            },
            hold_item1: ItemId::new(raw.value.item1),
            hold_item2: ItemId::new(raw.value.item2),
            gender_ratio: raw.value.gender_ratio,
            egg_cycles: raw.value.egg_cycles,
            base_friendship: raw.value.friendship,
            growth_rate: raw.value.growth_rate,
            egg_group1: map_egg_group("egg_group1", raw.value.egg_group1)?,
            egg_group2: if raw.value.egg_group1 == raw.value.egg_group2 {
                None
            } else {
                map_egg_group("egg_group2", raw.value.egg_group2)?
            },
            ability1: AbilityId::new(raw.value.ability1),
            ability2: AbilityId::new(raw.value.ability2),
            hidden_ability: AbilityId::new(raw.value.hidden_ability),
        })
    }
}

fn map_pokemon_type(raw: u8) -> PokemonType {
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
        x => PokemonType::Unknown,
    }
}

fn map_egg_group(field_name: &'static str, raw: u8) -> Result<Option<EggGroup>, ValidationError> {
    match raw {
        0x0 => Ok(None),
        0x1 => Ok(Some(EggGroup::Monster)),
        0x2 => Ok(Some(EggGroup::Water1)),
        0x3 => Ok(Some(EggGroup::Bug)),
        0x4 => Ok(Some(EggGroup::Flying)),
        0x5 => Ok(Some(EggGroup::Field)),
        0x6 => Ok(Some(EggGroup::Fairy)),
        0x7 => Ok(Some(EggGroup::Grass)),
        0x8 => Ok(Some(EggGroup::HumanLike)),
        0x9 => Ok(Some(EggGroup::Water3)),
        0xA => Ok(Some(EggGroup::Mineral)),
        0xB => Ok(Some(EggGroup::Amorphous)),
        0xC => Ok(Some(EggGroup::Water2)),
        0xD => Ok(Some(EggGroup::Ditto)),
        0xE => Ok(Some(EggGroup::Dragon)),
        0xF => Ok(Some(EggGroup::Undiscovered)),
        other => Err(ValidationError::new(
            field_name,
            format!("Invalid egg group 0x{:02X}", other),
        )),
    }
}
