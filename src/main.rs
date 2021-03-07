mod encoding;
mod rom;
mod save;

use std::fs::File;

use anyhow::{bail, Result};

use rom::Rom;
use save::Save;

const ROM_FILE: &str = "rom.bin";
const SAV_FILE: &str = "box.sav";

fn main() -> Result<()> {
    let rom = File::open(ROM_FILE).and_then(Rom::load)?;

    for i in 0..5 {
        let poke = rom.pokemon.get_by_species_id(i).unwrap();
        println!("Pokemon {}: {}", i, poke.name);
    }
    println!();

    let save = Save::read(File::open(SAV_FILE)?)?;
    println!("name:       {}", save.player_name);
    println!("gender:     {:?}", save.gender);
    let trainer_id = ((save.trainer_id[1] as u16) << 8) | (save.trainer_id[0] as u16);
    println!("trainer_id: {:04}", trainer_id);
    let total_seconds = save.play_time.as_secs();
    let hours = total_seconds / (60 * 60);
    let mins = (total_seconds - hours * 60 * 60) / 60;
    let secs = total_seconds - hours * 60 * 60 - mins * 60;
    println!("play_time:  {:02}:{:02}:{:02}", hours, mins, secs);

    println!("money:      {}", save.money);

    Ok(())
}
