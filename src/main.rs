mod encoding;
mod pokedex;
mod save;

use std::env;
use std::fs::File;

use anyhow::{bail, Result};

use save::Save;

fn main() -> Result<()> {
    let mut args = env::args();
    args.next();
    let file = match args.next() {
        Some(filename) => File::open(filename)?,
        None => bail!("Usage: leprogram <file>"),
    };

    let save = Save::read(file)?;

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
