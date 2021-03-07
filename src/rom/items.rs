use std::io::{self, Read, Seek, SeekFrom};

use byteorder::{LittleEndian, ReadBytesExt};

use super::read::RomReadExt;
use crate::encoding::parse_string_lossy;

const NUM_ITEMS: usize = 1268;
const ITEMS_OFFSET: u64 = 0x01C8;
const ITEM_NAME_LENGTH: usize = 13;

#[derive(Debug, Clone)]
pub struct ItemTable {
    items: Vec<Item>,
}

#[derive(Debug, Clone)]
pub struct Item {
    pub id: u16,
    pub name: String,
    pub price: u16,
}

impl ItemTable {
    pub fn load<R: Read + Seek>(mut reader: R) -> io::Result<Self> {
        reader.seek_pointer_at(ITEMS_OFFSET)?;
        let mut items = Vec::new();
        for _ in 0..NUM_ITEMS {
            let item = Item::load(&mut reader)?;
            items.push(item);
        }
        Ok(ItemTable { items })
    }

    pub fn get_by_item_id(&self, item_id: u16) -> Option<&Item> {
        self.items.get(item_id as usize)
    }
}

impl Item {
    pub fn load<R: Read + Seek>(mut reader: R) -> io::Result<Self> {
        let mut raw_name = [0u8; ITEM_NAME_LENGTH + 1];
        reader.read_exact(&mut raw_name)?;
        let name = parse_string_lossy(&raw_name);
        let id = reader.read_u16::<LittleEndian>()?;
        let price = reader.read_u16::<LittleEndian>()?;
        reader.seek(SeekFrom::Current(0x28 - 14))?;

        Ok(Item { id, name, price })
    }
}
