use std::fmt::{Debug};
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

pub trait Intimidating : Debug {
    fn intimidate(&self);
    fn fight(&self, character: &mut Character);
}

trait Monetary {
    fn dough(&self) -> u64;
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

    fn fight(&self, character: &mut Character) {
        character.skril += self.dough;
        println!("This guy just gave you his life savings ({})", self.dough);
        println!("say thank you sir");
    }
}

impl Monetary for Bureaucrat {
    fn dough(&self) -> u64 {
        self.dough
    }
}
