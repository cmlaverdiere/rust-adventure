use geography::{Coord, Jurisdiction};
use serde::Deserialize;
use std::fmt::Debug;

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
        character.skril += self.dough;

        match self.dough {
            0 => println!("this dude broke af lol"),
            _ => {
                println!("This guy just gave you his life savings ({})", self.dough);
                self.dough = 0;

                println!("say thank you sir");
            }
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
