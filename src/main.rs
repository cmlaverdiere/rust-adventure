// TODO Option to save progress to file and reload.
// TODO Message queue system for game events
//      - decouples frontend / printing from game logic
//      - enables testing of events fired instead of testing string messages
//      - can use pub/sub pattern if need mutliple consumers
// TODO Saving
//      (serialize / deserialize game data,
//       or replay commands (requires random seed))

mod creatures;
mod geography;
mod logger;
mod system;

use std::fs::{read_dir, File};
use std::io::{self, Write};
use std::{thread, time};

use clap::{App, Arg};
use serde::Deserialize;

use creatures::{Character, Sex};
use geography::{in_bounds, Cardinal, Coord, Land};
use logger::init_logger;
use system::prompt;

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
    // Barter, // TODO
    Combat,
    Movement,
    System,
    Repeat,
}

struct Desire {
    command: Command,
    args: Vec<String>,
}

const LEVEL_DATA_PATH: &str = "src/res/";

const READ_STRING_FAILURE: &str = "What? I didn't get that...";
const YES_VALUES: [&str; 9] = [
    "y", "yes", "yea", "yeah", "uh huh", "yeh", "sure", "why not", "totally",
];

const DRAMATIC_PAUSE: time::Duration = time::Duration::from_millis(500);

fn confirm(answer: &str) -> bool {
    YES_VALUES.contains(&answer.to_lowercase().as_str())
}

fn get_default_character() -> Character {
    let name = "Chris".to_string();

    Character {
        name,
        sex: Sex::Boy,
        whereabouts: None,
        skril: 0,
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
    land: &Land,
    args: Vec<String>,
) -> Result<(), String> {
    if args.len() != 2 {
        return Err("I just need a direction hoss.".to_string());
    }

    let direction_str = &args[1];
    let wb = character.whereabouts.unwrap();

    match direction_str.parse::<Cardinal>() {
        Ok(direction) => {
            let i_moved_here = yo_whered_i_move_to(&direction, wb)?;

            debug!("Moving character to {:?}.", i_moved_here);

            println!("You walk {:?}.", direction);
            character.whereabouts = Some((i_moved_here.0, i_moved_here.1));

            let new_plot = &mut land.plots.unwrap()[wb.1][wb.0];

            if new_plot.dosh != 0 {
                println!("You found {} zeni boi", new_plot.dosh);
                character.skril += new_plot.dosh;
                new_plot.dosh = 0;
            }

            if let Some(enemy) = new_plot.enemy {
                println!("uh oh there's someone here 🤪");
            }

            Ok(())
        }
        Err(_) => Err(format!("go {}? what the fuck", direction_str)),
    }
}

fn command_fight_enemy(character: &mut Character, land: &Land, _args: Vec<String>) {
    let wb = character.whereabouts.unwrap();

    debug!("Attempt to fight enemy at {:?}.", wb);

    match land.plots.unwrap()[wb.1][wb.0].enemy {
        Some(enemy) => {
            character.skril += enemy.dough;
            println!("This guy just gave you his life savings ({})", enemy.dough);
            println!("say thank you sir");
        }
        None => {
            println!("Ain't nobody around pal...");
        }
    }
}

fn this_guy_wants_to(input: &str) -> Result<Desire, &str> {
    let components = input.split(' ').collect::<Vec<&str>>();
    let root = components[0];

    let command = match root {
        "walk" | "go" | "run" => Ok(Command::Movement),
        "punch" | "kiss" | "lick" => Ok(Command::Combat),
        "quit" | "exit" => Ok(Command::System),
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
    let mut previous_commands: Vec<String> = Vec::new();

    loop {
        debug!("Character is at {:?}.", character.whereabouts.unwrap());

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
                } => match command_move_character(character, &land, args) {
                    Ok(_) => {}
                    Err(e) => {
                        println!("{}", e);
                    }
                },

                Desire {
                    command: Command::Combat,
                    args,
                } => command_fight_enemy(character, &land, args),

                Desire {
                    command: Command::System,
                    args,
                } => return,

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

        if character.skril > level.key_price {
            println!("You can get outa here now");
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
    if let Err(e) = init_logger() {
        println!("Failure initializing logger: {}", e);
        std::process::exit(1);
    }

    debug!("Game initializing.");

    let args = App::new("Adventure")
        .version("0.1")
        .author("Chris Laverdiere <cmlaverdiere@gmail.com>")
        .about("A text-based adventure game")
        .arg(Arg::with_name("skip-character-creation").short("s"))
        .get_matches();

    debug!("Arguments parsed.");

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
