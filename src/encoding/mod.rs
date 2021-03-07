pub mod charmap;

use charmap::{Encoding, CHARMAP};

pub fn parse_string_lossy(raw: &[u8]) -> String {
    raw.iter()
        .map(|c| match CHARMAP[*c as usize] {
            Encoding::Char(c) => Some(c),
            Encoding::Invalid => Some('\u{FFFD}'),
            Encoding::End => None,
        })
        .take_while(|c| !c.is_none())
        .filter_map(|c| c)
        .collect()
}
