use geography::{Coord, Jurisdiction};
use serde::Deserialize;

#[derive(Debug)]
pub enum Sex {
    Boy,
    Girl,
}

pub struct Character {
    pub name: String,
    pub sex: Sex,
    pub whereabouts: Option<Coord>,
    pub skril: u64,
}

trait Enemy {
    fn dough(&self) -> u64;
    fn intimidate(&self);
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

impl Enemy for Bureaucrat {
    fn intimidate(&self) {
        println!("I'm gon raise yo taxes boy");
    }

    fn dough(&self) -> u64 {
        self.dough
    }
}
