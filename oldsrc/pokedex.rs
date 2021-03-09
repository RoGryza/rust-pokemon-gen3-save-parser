use std::collections::HashMap;
use std::fmt::{self, Display, Formatter};

pub type Pokedex = HashMap<NationalDexId, PokedexStatus>;

#[derive(Debug, Copy, Clone)]
pub enum PokedexStatus {
    Seen,
    Caught,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct SpeciesId(pub u16);

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct NationalDexId(pub u16);

impl Display for SpeciesId {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl From<u16> for SpeciesId {
    fn from(id: u16) -> Self {
        SpeciesId(id)
    }
}

impl Display for NationalDexId {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl From<u16> for NationalDexId {
    fn from(id: u16) -> Self {
        NationalDexId(id)
    }
}
