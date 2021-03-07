mod encoding;
mod pokedex;
mod rom;
mod save;

use std::fs::File;

use anyhow::Result;

use rom::Rom;
use save::Save;

const ROM_FILE: &str = "rom.bin";
const SAV_FILE: &str = "box.sav";

fn main() -> Result<()> {
    env_logger::init();

    let rom = File::open(ROM_FILE).and_then(Rom::load)?;

    for i in 0..5 {
        let poke = rom.pokemon.get_by_species_id(i.into()).unwrap();
        println!("Pokemon {}: {}", i, poke.name);
    }
    println!();

    for i in 0..5 {
        let mv = rom.moves.get_by_move_id(i).unwrap();
        println!("Move {}: {}", i, mv.name);
    }
    println!();

    for i in 0..5 {
        let ability = rom.abilities.get_by_ability_id(i).unwrap();
        println!("Ability {}: {}", i, ability.name);
    }
    println!();

    for i in 0..5 {
        let item = rom.items.get_by_item_id(i).unwrap();
        println!("Item {}: {}", i, item.name);
        println!("     id: {}", item.id);
        println!("  price: {}", item.price);
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
    println!("{} pokemon in pokedex:", save.pokedex.len());
    for (national_dex_id, status) in &save.pokedex {
        let species_id = rom
            .pokemon
            .national_dex_to_species_id(*national_dex_id)
            .unwrap();
        let poke = rom.pokemon.get_by_species_id(species_id).unwrap();
        println!("  {}: {:?}", poke.name, status);
    }

    Ok(())
}
