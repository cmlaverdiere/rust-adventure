use geography::{Coord, Jurisdiction};
use rand::Rng;
use serde::Deserialize;
use std::fmt::Debug;

const FIGHT_WIN_CHANCE: f32 = 0.8;

#[derive(Debug)]
pub enum Sex {
    Boy,
    Girl,
}

#[derive(Debug)]
pub struct Character {
    pub name: String,
    pub sex: Sex,
    pub whereabouts: Option<Coord>,
    pub skril: u64,
    pub stats: Stats,
}

#[derive(Debug)]
pub struct Stats {
    pub muscles: u64,
    pub brains: u64,
    pub mojo: u64,
}

pub trait Intimidating: Debug {
    fn intimidate(&self);
    fn fight(&mut self, character: &mut Character);
}

pub trait Monetary {
    fn dough(&self) -> u64;
    fn take_payment(&mut self, amount: u64);
}

#[derive(Copy, Clone, Debug, Deserialize)]
pub struct Bureaucrat {
    pub dough: u64,
    pub jurisdiction: Jurisdiction,
}

impl Bureaucrat {
    fn new(dough: u64, jurisdiction: Jurisdiction) -> Bureaucrat {
        Bureaucrat {
            dough,
            jurisdiction,
        }
    }
}

impl Intimidating for Bureaucrat {
    fn intimidate(&self) {
        println!("I'm gon raise yo taxes boy");
    }

    fn fight(&mut self, character: &mut Character) {
        self.intimidate();

        let rng = &mut rand::thread_rng();
        let win_chance = rng.gen_range(0.0, 1.0);

        if win_chance < FIGHT_WIN_CHANCE {
            debug!("Won fight against Bureaucrat.");

            character.skril += self.dough;

            match self.dough {
                0 => println!("this dude broke af lol"),
                _ => {
                    println!("This guy just gave you his life savings ({})", self.dough);
                    self.dough = 0;

                    println!("say thank you sir");
                }
            }
        } else {
            println!("u fokin lost m8");
        }
    }
}

// TODO decouple messages from this module
impl Monetary for &mut Bureaucrat {
    fn dough(&self) -> u64 {
        self.dough
    }

    fn take_payment(&mut self, amount: u64) {
        println!("Bureaucrat got more money... 😞");
        self.dough += amount;
    }
}

#[derive(Default, Debug)]
pub struct PersonalTrainer {
    pub dough: u64,
}

impl Monetary for &mut PersonalTrainer {
    fn dough(&self) -> u64 {
        self.dough
    }

    fn take_payment(&mut self, amount: u64) {
        println!("Personal trainer got paid, but money can't buy gains");
        self.dough += amount;
    }
}

#[derive(Default, Debug)]
pub struct UberDriver {
    pub fee: u64,
    pub rating: f32,
}

impl UberDriver {
    pub fn initiate_ride(&self, amount: u64) -> Result<String, String> {
        if amount >= self.fee {
            Ok("Uber driver got slightly richer".to_string())
        } else {
            Err("TOO POOR".to_string())
        }
    }
}

pub mod entity {
    pub const TAXI: &str = "taxi";
    pub const ENEMY: &str = "enemy";
    pub const PERSONAL_TRAINER: &str = "trainer";
}
