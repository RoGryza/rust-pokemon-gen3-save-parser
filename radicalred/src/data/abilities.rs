use std::convert::TryFrom;

use poke3_common::encoding::parse_string;
use poke3_common::rom::Ability;

use super::ValidationError;
use crate::read::FromTable;

#[repr(transparent)]
pub struct RawAbilityName([u8; 17]);

impl FromTable for RawAbilityName {
    const NAME: &'static str = "gAbilityNames";
    const COUNT: usize = 0xFE + 1;
    const OFFSET: u64 = 0x01C0;
}

impl TryFrom<RawAbilityName> for Ability {
    type Error = ValidationError;

    fn try_from(raw: RawAbilityName) -> Result<Ability, ValidationError> {
        Ok(Ability {
            name: parse_string(&raw.0).map_err(|e| ValidationError::from_display("name", e))?,
        })
    }
}
