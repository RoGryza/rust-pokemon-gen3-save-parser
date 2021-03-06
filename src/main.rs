mod encoding;
mod save;

use std::env;
use std::fs::File;

use anyhow::{bail, Result};

use save::section::{SaveBlock2, SAVE_SECTION_SECTORS};
use save::Sector;

fn main() -> Result<()> {
    let mut args = env::args();
    args.next();
    let file = match args.next() {
        Some(filename) => File::open(filename)?,
        None => bail!("Usage: leprogram <file>"),
    };

    for save_slot in 0..2 {
        let start = save_slot * SAVE_SECTION_SECTORS;
        for i in start..start + SAVE_SECTION_SECTORS {
            let sector = Sector::read(&file)?;
            if sector.id == 0 {
                println!("SaveBlock2 slot {} at sector {}", save_slot, i);
                match SaveBlock2::from_sector(&sector) {
                    Ok(block) => {
                        println!("  name:       {}", block.player_name);
                        println!("  gender:     {:?}", block.gender);
                        let trainer_id =
                            ((block.trainer_id[1] as u16) << 8) | (block.trainer_id[0] as u16);
                        println!("  trainer_id: {:04}", trainer_id);
                        let total_seconds = block.play_time.as_secs();
                        let hours = total_seconds / (60 * 60);
                        let mins = (total_seconds - hours * 60 * 60) / 60;
                        let secs = total_seconds - hours * 60 * 60 - mins * 60;
                        println!("  play_time:  {:02}:{:02}:{:02}", hours, mins, secs);
                    }
                    Err(e) => println!("  Error: {}", e),
                }
            }
        }
    }

    Ok(())
}
