mod abilities;
mod items;
mod species;

use std::convert::TryInto;
use std::fmt::Display;
use std::io::{Read, Seek};

use poke3_common::rom::Rom;

use crate::read::{ReadTableError, ReadTableResult, RomReadExt};
use abilities::RawAbilityName;
use items::RawItem;
use species::RawPokemon;

#[derive(Debug)]
pub struct ValidationError {
    field: &'static str,
    message: String,
}

impl ValidationError {
    pub fn new<S: Into<String>>(field: &'static str, message: S) -> Self {
        ValidationError {
            field,
            message: message.into(),
        }
    }

    pub fn from_display<S: Display>(field: &'static str, message: S) -> Self {
        ValidationError {
            field,
            message: message.to_string(),
        }
    }
}

#[derive(Debug)]
pub enum ReadRomError {
    ReadTable(ReadTableError),
    Validation(ValidationError),
}

pub type ReadRomResult<T> = Result<T, ReadRomError>;

impl From<ReadTableError> for ReadRomError {
    fn from(err: ReadTableError) -> Self {
        ReadRomError::ReadTable(err)
    }
}

impl From<ValidationError> for ReadRomError {
    fn from(err: ValidationError) -> Self {
        ReadRomError::Validation(err)
    }
}

pub fn read_rom<R: Read + Seek>(mut reader: R) -> ReadRomResult<Rom> {
    let rom = Rom {
        abilities: try_into_iter(reader.read_table::<RawAbilityName>()?)?,
        items: try_into_iter(reader.read_table::<RawItem>()?)?,
        moves: Vec::new(),
        species: try_into_iter(RawPokemon::read_all(&mut reader)?)?,
    };
    Ok(rom)
}

fn try_into_iter<I, T, U>(iter: I) -> ReadRomResult<Vec<U>>
where
    I: Iterator<Item = ReadTableResult<T>>,
    T: TryInto<U, Error = ValidationError>,
{
    iter.map(|r| {
        r.map_err(ReadRomError::from)
            .and_then(|raw| raw.try_into().map_err(ReadRomError::from))
    })
    .collect()
}
