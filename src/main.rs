// TODO Make letters print slowly as if a video game
// TODO Message queue system for game events
//      - decouples frontend / printing from game logic
//      - enables testing of events fired instead of testing string messages
//      - can use pub/sub pattern if need multiple consumers
// TODO Saving
//      (serialize / deserialize game data,
//       or replay commands (requires random seed))
// TODO Character stats and leveling up
// TODO Seed for random to allow testing? Or mocks?

mod creatures;
mod geography;
mod logger;
mod system;

use rand::Rng;
use std::fs::{read_dir, File};
use std::io::{self, Write};
use std::{thread, time};

use clap::{App, Arg};
use serde::Deserialize;

use creatures::{Character, Monetary, Sex, Stats};
use geography::{in_bounds, Cardinal, Coord, Land};
use logger::init_logger;
use system::{delay_print, prompt};

#[macro_use]
extern crate log;
extern crate clap;
extern crate rand;
extern crate serde;

// enum Event {
//     ENEMY_INTIMIDATED_PLAYER,
//     TIME_ADVANCED,
//     CHARACTER_CREATED,
// }

#[derive(Debug, Deserialize)]
struct Level {
    name: String,
    level_id: u64,
    key_price: u64,
}

enum Command {
    Combat,
    Movement,
    Repeat,
    Riding,
    System,
    Train,
    Debug,
}

struct Desire {
    command: Command,
    args: Vec<String>,
}

const LEVEL_DATA_PATH: &str = "src/res/";

const YES_VALUES: [&str; 17] = [
    "y",
    "yes",
    "yea",
    "yeah",
    "uh huh",
    "yeh",
    "sure",
    "why not",
    "totally",
    "okay",
    "ok",
    "i guess",
    "whatever",
    "hell yeah",
    "fuck yeah",
    "fuck yes",
    "yes oh lord",
];

const DRAMATIC_PAUSE: time::Duration = time::Duration::from_millis(500);

fn confirm(answer: &str) -> bool {
    YES_VALUES.contains(&answer.to_lowercase().as_str())
}

fn get_default_character() -> Character {
    let name = "Bobby Hill".to_string();

    Character {
        name,
        sex: Sex::Boy,
        whereabouts: None,
        skril: 0,
        stats: Stats {
            muscles: 0,
            brains: 0,
            mojo: 0,
        },
    }
}

fn create_character() -> Character {
    let mut name;

    println!("HEY!");
    println!("What did you say your name was again?");

    let mut repeating_question = false;
    loop {
        if repeating_question {
            for _ in 0..3 {
                print!(".");
                io::stdout().flush().unwrap();
                thread::sleep(DRAMATIC_PAUSE)
            }
            println!();
            println!("So what is it then.");
        }

        name = prompt();

        if !repeating_question {
            println!("{}...? Really?", name);
        } else {
            println!("You sure?");
        }

        let name_confirmation = prompt();

        if confirm(&name_confirmation) {
            println!("Got it, {}.", name);
            break;
        } else {
            repeating_question = true;
        }
    }

    println!("And you're a boy... right?");
    let sex_confirmation = prompt();
    let sex = if confirm(&sex_confirmation) {
        Sex::Boy
    } else {
        Sex::Girl
    };

    println!("...right.");
    println!("If there's nothing else...");
    prompt();

    Character {
        name,
        sex,
        whereabouts: None,
        skril: 0,
        stats: Stats {
            muscles: 0,
            brains: 0,
            mojo: 0,
        },
    }
}

fn read_levels() -> Vec<Level> {
    debug!("Reading level data.");
    let files = read_dir(LEVEL_DATA_PATH)
        .expect("Level directory not found.")
        .flatten();
    let levels: Vec<Level> = files
        .map(|f| {
            let file = File::open(f.path()).expect("Failed to open level data file.");
            let level: Level = serde_json::from_reader(file).expect("File is not valid JSON.");
            level
        })
        .collect();
    levels
}

fn command_move_character(
    character: &mut Character,
    land: &mut Land,
    args: Vec<String>,
) -> Result<(), String> {
    if args.len() != 2 {
        return Err("I just need a direction hoss.".to_string());
    }

    let direction_str = &args[1];
    let starting_wb = character.whereabouts.unwrap();

    match direction_str.parse::<Cardinal>() {
        Ok(direction) => {
            let i_moved_here = yo_whered_i_move_to(&direction, starting_wb)?;

            println!("You walk {:?}.", direction);
            character.whereabouts = Some((i_moved_here.0, i_moved_here.1));

            let new_plot = &mut land.plots.as_mut().unwrap()[i_moved_here.0][i_moved_here.1];
            debug!("New plot moved to: {:?}", new_plot);

            if new_plot.dosh != 0 {
                println!("You found {} zeni boi", new_plot.dosh);
                character.skril += new_plot.dosh;
                new_plot.dosh = 0;
            }

            if let Some(ref _enemy) = new_plot.enemy {
                println!("uh oh there's a baddie here 🤪");
            }

            if let Some(ref _driver) = new_plot.driver {
                println!("oh look an uber");
            }

            if let Some(ref _trainer) = new_plot.trainer {
                println!("is that a personal trainer? he looks small and poor");
            }

            Ok(())
        }
        Err(_) => Err(format!("go {}? what the fuck", direction_str)),
    }
}

fn command_train(
    character: &mut Character,
    land: &mut Land,
    _args: Vec<String>,
) -> Result<(), String> {
    let wb = character.whereabouts.unwrap();

    debug!("Attempt to train at {:?}.", wb);

    match &mut land.plots.as_mut().unwrap()[wb.0][wb.1].trainer.as_mut() {
        Some(trainer) => {
            // TODO train, increase stats, printout each line
            let cost = ((character.stats.muscles + character.stats.mojo + character.stats.brains)
                as f64)
                .sqrt() as u64;
            if cost == 0 {
                println!("First session's free, you ready?");
            } else {
                println!("You can train but it'll cost you {}, capiche?", cost);
            }

            let response = prompt();
            if confirm(&response) {
                if character.skril >= cost {
                    trainer.take_payment(cost);
                    character.skril -= cost;

                    let rng = &mut rand::thread_rng();
                    let muscle_inc = rng.gen_range(3, 12);
                    print!("{:💪<1$} ", "", muscle_inc as usize);
                    print!(
                        "Muscles {} -> {}",
                        character.stats.muscles,
                        character.stats.muscles + muscle_inc,
                    );
                    println!(" {:💪<1$}", "", muscle_inc as usize);
                    character.stats.muscles += muscle_inc;
                } else {
                    println!("You can't afford this. Get a job.");
                }
            }

            Ok(())
        }
        None => Err("No gains to be found here bub".to_string()),
    }
}

fn command_fight_enemy(
    character: &mut Character,
    land: &mut Land,
    _args: Vec<String>,
) -> Result<(), String> {
    let wb = character.whereabouts.unwrap();

    debug!("Attempt to fight enemy at {:?}.", wb);

    match &mut land.plots.as_mut().unwrap()[wb.0][wb.1].enemy {
        Some(enemy) => {
            enemy.fight(character);
            Ok(())
        }
        None => Err("Ain't nobody around pal...".to_string()),
    }
}

fn command_rideshare(
    character: &mut Character,
    land: &mut Land,
    _args: Vec<String>,
) -> Result<(), String> {
    let wb = character.whereabouts.unwrap();

    debug!("Attempt to rideshare at {:?}.", wb);

    match &mut land.plots.as_mut().unwrap()[wb.0][wb.1].driver {
        Some(driver) => match driver.initiate_ride(character.skril) {
            Ok(msg) => {
                println!("{}", msg);
                Ok(())
            }
            Err(msg) => Err(msg),
        },
        None => Err("gotta find a taxi first bub".to_string()),
    }
}

fn command_debug_info(
    character: &mut Character,
    land: &mut Land,
    args: Vec<String>,
) -> Result<(), String> {
    if args.len() != 2 {
        return Err("gotta tell me what to print out hoss".to_string());
    }

    let obj = &args[1];
    match land.entity_locations.as_ref().unwrap().get(obj.as_str()) {
        Some(entities) => println!("{:?}", entities),
        None => println!("None of whatever those are around."),
    }

    Ok(())
}

fn this_guy_wants_to(input: &str) -> Result<Desire, &str> {
    let components = input.split(' ').collect::<Vec<&str>>();
    let root = components[0];

    let command = match root {
        "walk" | "go" | "run" => Ok(Command::Movement),
        "take" | "ride" | "get in" => Ok(Command::Riding),
        "punch" | "fight" | "lick" => Ok(Command::Combat),
        "train" | "lift" | "exercise" | "workout" | "work out" => Ok(Command::Train),
        "quit" | "bounce" => Ok(Command::System),
        "debug" | "dbg" | "p" => Ok(Command::Debug),
        "" => Ok(Command::Repeat),
        _ => Err("Tf?"),
    };

    match command {
        Ok(command) => Ok(Desire {
            command,
            args: components.into_iter().map(String::from).collect(),
        }),
        Err(e) => Err(e),
    }
}

fn init_adventure(character: &mut Character) {
    println!(
        "{}, {:?}, you wake up in an unfamiliar land.",
        character.name, character.sex
    );
    println!("You have no food nor water. You are naked.");
    println!("Your leg hurts.");

    let levels = read_levels();
    assert!(
        !levels.is_empty(),
        "Your game level data is corrupted... what did you do"
    );
    let level = &levels[0];

    let mut land = Land::new();
    land.init_plots();

    character.whereabouts = Some((0, 0));

    println!("Cha wanna do now?");

    let mut repeat_last_command = false;
    let mut alerted_exit = false;
    let mut previous_commands: Vec<String> = Vec::new();

    loop {
        debug!("{:?}", character);

        let character_desire = if !repeat_last_command {
            prompt().to_lowercase()
        } else {
            repeat_last_command = false;
            previous_commands.pop().unwrap()
        };

        match this_guy_wants_to(character_desire.as_ref()) {
            Ok(desire) => match desire {
                Desire {
                    command: Command::Movement,
                    args,
                } => match command_move_character(character, &mut land, args) {
                    Ok(_) => {}
                    Err(e) => {
                        println!("{}", e);
                    }
                },

                Desire {
                    command: Command::Train,
                    args,
                } => match command_train(character, &mut land, args) {
                    Ok(_) => {}
                    Err(e) => {
                        println!("{}", e);
                    }
                },

                Desire {
                    command: Command::Combat,
                    args,
                } => match command_fight_enemy(character, &mut land, args) {
                    Ok(_) => {}
                    Err(e) => {
                        println!("{}", e);
                    }
                },

                Desire {
                    command: Command::Riding,
                    args,
                } => match command_rideshare(character, &mut land, args) {
                    Ok(_) => {
                        // TODO load new level, function to load level
                        println!("GAME OVER - YOU WIN");
                        return;
                    }
                    Err(e) => {
                        println!("{}", e);
                    }
                },

                Desire {
                    command: Command::System,
                    args,
                } => return,

                Desire {
                    command: Command::Debug,
                    args,
                } => match command_debug_info(character, &mut land, args) {
                    Ok(_) => {}
                    Err(e) => {
                        println!("{}", e);
                    }
                },

                Desire {
                    command: Command::Repeat,
                    args,
                } => {
                    repeat_last_command = true;
                    continue;
                }
            },
            Err(e) => {
                println!("{}", e);
            }
        }

        if character.skril > level.key_price && !alerted_exit {
            println!("You can get outa here now");
            alerted_exit = true;
        }

        previous_commands.push(character_desire);
    }
}

fn yo_whered_i_move_to(direction: &Cardinal, wb: Coord) -> Result<Coord, &'static str> {
    let i_moved_here: (isize, isize);

    match direction {
        Cardinal::North => i_moved_here = (wb.0 as isize, wb.1 as isize + 1 as isize),
        Cardinal::East => i_moved_here = (wb.0 as isize + 1 as isize, wb.1 as isize),
        Cardinal::South => i_moved_here = (wb.0 as isize, wb.1 as isize - 1 as isize),
        Cardinal::West => i_moved_here = (wb.0 as isize - 1 as isize, wb.1 as isize),
    }

    if !in_bounds(i_moved_here) {
        Err("You hit a wall.")
    } else {
        Ok((i_moved_here.0 as usize, i_moved_here.1 as usize))
    }
}

fn main() {
    let args = App::new("Adventure")
        .version("0.1")
        .author("Chris Laverdiere <cmlaverdiere@gmail.com>")
        .about("A text-based adventure game")
        .arg(Arg::with_name("skip-character-creation").short("s"))
        .arg(Arg::with_name("debug").short("d"))
        .get_matches();

    debug!("Arguments parsed.");

    if let Err(e) = init_logger(args.is_present("debug")) {
        println!("Failure initializing logger: {}", e);
        std::process::exit(1);
    }

    debug!("Game initializing.");
    let mut character = if !args.is_present("skip-character-creation") {
        create_character()
    } else {
        get_default_character()
    };

    debug!("Character created.");

    init_adventure(&mut character);

    debug!("Exiting game.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_level_data() {
        let levels = read_levels();
        assert_eq!(levels.len(), 1);
        assert_eq!(levels[0].level_id, 1);
    }
}
