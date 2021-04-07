use std::convert::TryFrom;

use poke3_common::encoding::parse_string;
use poke3_common::rom::{Item, ItemPocket, ItemType};

use super::ValidationError;
use crate::read::FromTable;

#[repr(C, align(1))]
pub struct RawItem {
    name: [u8; 14],
    _item_id: u16,
    price: u16,
    _hold_effect: u8,
    _hold_effect_param: u8,
    _description: u32, // *const u8
    _importance: u8,
    _unk19: u8,
    pocket: u8,
    item_type: u8,
    _field_use_func: u32, // ItemUseFunc
    _battle_usage: u8,
    _battle_use_func: u32, // ItemUseFunc
    _secondary_id: u8,
}

impl FromTable for RawItem {
    const NAME: &'static str = "gItems";
    const COUNT: usize = 250;
    const OFFSET: u64 = 0x01C8;
}

impl TryFrom<RawItem> for Item {
    type Error = ValidationError;

    fn try_from(raw: RawItem) -> Result<Item, ValidationError> {
        Ok(Item {
            name: parse_string(&raw.name).map_err(|e| ValidationError::from_display("name", e))?,
            price: raw.price,
            pocket: map_pocket(raw.pocket)?,
            item_type: map_item_type(raw.item_type)?,
        })
    }
}

fn map_pocket(raw: u8) -> Result<ItemPocket, ValidationError> {
    match raw {
        1 => Ok(ItemPocket::Items),
        2 => Ok(ItemPocket::KeyItems),
        3 => Ok(ItemPocket::PokeBalls),
        4 => Ok(ItemPocket::TmCase),
        5 => Ok(ItemPocket::BerryPouch),
        other => Err(ValidationError::new(
            "pocket",
            format!("Invalid item pocket {}", other),
        )),
    }
}

fn map_item_type(raw: u8) -> Result<ItemType, ValidationError> {
    match raw {
        0x00 => Ok(ItemType::FieldUse),
        0x01 => Ok(ItemType::HealthRecovery),
        0x02 => Ok(ItemType::StatusRecovery),
        0x03 => Ok(ItemType::PpRecovery),
        0x03 => Ok(ItemType::StatBoostDrink),
        0x04 => Ok(ItemType::StatBoostWing),
        0x05 => Ok(ItemType::EvolutionStone),
        0x06 => Ok(ItemType::EvolutionItem),
        0x07 => Ok(ItemType::BattleItem),
        0x08 => Ok(ItemType::Flute),
        0x09 => Ok(ItemType::StatBoostHeldItem),
        0x0A => Ok(ItemType::HeldItem),
        0x0B => Ok(ItemType::Gem),
        0x0C => Ok(ItemType::Plate),
        0x0D => Ok(ItemType::Memory),
        0x0E => Ok(ItemType::Drive),
        0x0F => Ok(ItemType::Incense),
        0x10 => Ok(ItemType::MegaStone),
        0x11 => Ok(ItemType::ZCrystal),
        0x12 => Ok(ItemType::Nectar),
        0x13 => Ok(ItemType::Sellable),
        0x14 => Ok(ItemType::Relic),
        0x15 => Ok(ItemType::Shard),
        0x16 => Ok(ItemType::Fossil),
        0x17 => Ok(ItemType::Mail),
        other => Err(ValidationError::new(
            "type",
            format!("Invalid item type {}", other),
        )),
    }
}
