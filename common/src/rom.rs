use serde::{Deserialize, Serialize};
use std::num::{NonZeroU16, NonZeroU8};

pub type AbilityId = NonZeroU8;
pub type ItemId = NonZeroU16;

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
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
    Unknown,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum EggGroup {
    Monster,
    Water1,
    Bug,
    Flying,
    Field,
    Fairy,
    Grass,
    HumanLike,
    Water3,
    Mineral,
    Amorphous,
    Water2,
    Ditto,
    Dragon,
    Undiscovered,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Split {
    Physical,
    Special,
    Status,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ItemPocket {
    Items,
    KeyItems,
    PokeBalls,
    TmCase,
    BerryPouch,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ItemType {
    FieldUse,
    HealthRecovery,
    StatusRecovery,
    PpRecovery,
    StatBoostDrink,
    StatBoostWing,
    EvolutionStone,
    EvolutionItem,
    BattleItem,
    Flute,
    StatBoostHeldItem,
    HeldItem,
    Gem,
    Plate,
    Memory,
    Drive,
    Incense,
    MegaStone,
    ZCrystal,
    Nectar,
    Sellable,
    Relic,
    Shard,
    Fossil,
    Mail,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rom {
    pub abilities: Vec<Ability>,
    pub items: Vec<Item>,
    pub moves: Vec<Move>,
    pub species: Vec<Species>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ability {
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    pub name: String,
    pub price: u16,
    pub pocket: ItemPocket,
    pub item_type: ItemType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Move {
    pub name: String,
    pub power: u8,
    pub move_type: PokemonType,
    pub accuracy: u8,
    pub pp: u8,
    pub secondary_effect_chance: u8,
    pub priority: u8,
    pub split: Split,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Species {
    pub original_name: Option<String>,
    pub ingame_name: String,
    pub base_stats: Stats<u8>,
    pub type1: PokemonType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub type2: Option<PokemonType>,
    pub catch_rate: u8,
    pub exp: u8,
    pub ev_yield: Stats<u8>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hold_item1: Option<ItemId>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hold_item2: Option<ItemId>,
    pub gender_ratio: u8,
    pub egg_cycles: u8,
    pub base_friendship: u8,
    pub growth_rate: u8,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub egg_group1: Option<EggGroup>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub egg_group2: Option<EggGroup>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ability1: Option<AbilityId>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ability2: Option<AbilityId>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hidden_ability: Option<AbilityId>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stats<T> {
    pub hp: T,
    pub atk: T,
    pub def: T,
    pub spa: T,
    pub spd: T,
    pub spe: T,
}
