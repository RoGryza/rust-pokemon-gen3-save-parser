use std::io::{self, Read, Seek};

use super::read::RomReadExt;

const NUM_MOVES: usize = 0x37F + 1;
const MOVE_NAME_LENGTH: usize = 12;
const MOVE_NAMES_OFFSET: u64 = 0x04EF84;

#[derive(Debug, Clone)]
pub struct MoveTable {
    moves: Vec<Move>,
}

#[derive(Debug, Clone)]
pub struct Move {
    pub name: String,
}

impl MoveTable {
    pub fn load<R: Read + Seek>(mut reader: R) -> io::Result<Self> {
        reader.seek_pointer_at(MOVE_NAMES_OFFSET)?;
        let mut moves = Vec::new();
        for name_res in reader.read_string_list(NUM_MOVES, MOVE_NAME_LENGTH + 1) {
            let name = name_res?;
            moves.push(Move { name });
        }
        Ok(MoveTable { moves })
    }

    pub fn get_by_move_id(&self, move_id: u16) -> Option<&Move> {
        self.moves.get(move_id as usize)
    }
}
