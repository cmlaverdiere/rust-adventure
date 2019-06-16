// TODO Option to save progress to file and reload.
// TODO git repo
// TODO type alias (usize, usize)

use rand::Rng;
use std::fs::File;
use std::io::{self, Write};
use std::str::FromStr;
use std::{thread, time};

use clap::{App, Arg};
use serde::Deserialize;

extern crate clap;
extern crate rand;
extern crate serde;

struct Character {
    name: String,
    sex: Sex,
    whereabouts: Option<(usize, usize)>,
    skril: u64,
}

struct Land {
    plots: Option<[[Plot; LAND_WIDTH_X]; LAND_WIDTH_Y]>,
}

impl Land {
    pub fn new() -> Land {
        Land { plots: None }
    }

    pub fn init_plots(&mut self) {
        let rng = &mut rand::thread_rng();

        self.plots = Some([[Plot { dosh: 0 }; LAND_WIDTH_X]; LAND_WIDTH_Y]);
        if let Some(mut plots) = self.plots {
            for i in 0..LAND_WIDTH_X {
                for j in 0..LAND_WIDTH_Y {
                    if rng.gen_range(0.0, 1.0) > ZENI_GEN_CHANCE {
                        plots[i][j].dosh = rng.gen_range(0, 10)
                    }
                }
            }
            self.plots = Some(plots);
        }
    }
}

#[derive(Copy, Clone, Debug, Deserialize)]
struct Plot {
    dosh: u64,
}

#[derive(Debug, Deserialize)]
struct Level {
    name: String,
    level_id: u32,
    key_price: u32,
}

#[derive(Debug)]
enum Sex {
    Boy,
    Girl,
}

#[derive(Debug)]
enum Cardinal {
    North,
    East,
    South,
    West,
}

impl FromStr for Cardinal {
    type Err = ();

    fn from_str(s: &str) -> Result<Cardinal, ()> {
        match s {
            "north" => Ok(Cardinal::North),
            "east" => Ok(Cardinal::East),
            "south" => Ok(Cardinal::South),
            "west" => Ok(Cardinal::West),
            _ => Err(()),
        }
    }
}

const READ_STRING_FAILURE: &str = "What? I didn't get that...";
const YES_VALUES: [&str; 9] = [
    "y", "yes", "yea", "yeah", "uh huh", "yeh", "sure", "why not", "totally",
];
const DRAMATIC_PAUSE: time::Duration = time::Duration::from_millis(500);

const LAND_WIDTH_X: usize = 10;
const LAND_WIDTH_Y: usize = 10;
const ZENI_GEN_CHANCE: f32 = 0.8;

fn prompt() -> String {
    let mut result = String::new();

    print!("> ");
    io::stdout().flush().unwrap();

    io::stdin()
        .read_line(&mut result)
        .expect(READ_STRING_FAILURE);

    result.pop();

    result
}

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

fn read_level() -> Level {
    let file = File::open("src/res/level1.json").expect("Failed to open level data file.");
    let level: Level = serde_json::from_reader(file).expect("File is not valid JSON.");
    level
}

fn init_adventure(character: &mut Character) {
    println!(
        "{}, {:?}, you wake up in an unfamiliar land.",
        character.name, character.sex
    );
    println!("You have no food nor water. You are naked.");
    println!("Your leg hurts.");

    let _level = read_level();

    // Initialize land.
    let mut land = Land::new();
    land.init_plots();

    character.whereabouts = Some((0, 0));

    println!("Pick a direction {{north, south, east, west}}.");
    // TODO Turn into shell

    loop {
        let direction_str = prompt().to_lowercase();

        let wb = character.whereabouts.unwrap();

        match direction_str.parse::<Cardinal>() {
            Ok(direction) => {
                match calc_wb(&direction, wb) {
                    Ok(new_wb) => {
                        println!("You walk {:?}.", direction);
                        character.whereabouts = Some((new_wb.0, new_wb.1));

                        let new_plot = &mut land.plots.unwrap()[wb.1][wb.0];

                        if new_plot.dosh != 0 {
                            println!("You found {} zeni boi", new_plot.dosh);
                            character.skril += new_plot.dosh;
                            new_plot.dosh = 0;
                        }
                    }
                    Err(e) => {
                        println!("{}", e);
                    }
                }
            }
            Err(_) => {
                println!("tf?");
            }
        }
    }
}

fn calc_wb(direction: &Cardinal, wb: (usize, usize)) -> Result<(usize, usize), &str> {
    let new_wb: (isize, isize);

    match direction {
        Cardinal::North => new_wb = (wb.0 as isize, wb.1 as isize + 1 as isize),
        Cardinal::East => new_wb = (wb.0 as isize + 1 as isize, wb.1 as isize),
        Cardinal::South => new_wb = (wb.0 as isize, wb.1 as isize - 1 as isize),
        Cardinal::West => new_wb = (wb.0 as isize - 1 as isize, wb.1 as isize),
    }

    if !in_bounds(new_wb) {
        Err("You hit a wall.")
    } else {
        Ok((new_wb.0 as usize, new_wb.1 as usize))
    }
}

fn in_bounds(wb: (isize, isize)) -> bool {
    wb.0 >= 0 && wb.1 >= 0 && wb.0 < (LAND_WIDTH_X as isize) && wb.1 < (LAND_WIDTH_Y as isize)
}

fn main() {
    let args = App::new("Adventure")
        .version("0.1")
        .author("Chris Laverdiere <cmlaverdiere@gmail.com>")
        .about("A text-based adventure game")
        .arg(Arg::with_name("skip-character-creation").short("s"))
        .get_matches();

    let mut character = if !args.is_present("skip-character-creation") {
        create_character()
    } else {
        get_default_character()
    };

    init_adventure(&mut character);
}
