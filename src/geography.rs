// TODO refactor game logic out of this module.
use rand::Rng;
use std::str::FromStr;

use creatures::{Bureaucrat, Intimidating};
use serde::Deserialize;

const LAND_WIDTH_X: usize = 10;
const LAND_WIDTH_Y: usize = 10;

const ZENI_GEN_CHANCE: f32 = 0.3;
const ENEMY_GEN_CHANCE: f32 = 0.2;

pub type Coord = (usize, usize);

pub fn in_bounds(c: (isize, isize)) -> bool {
    c.0 >= 0 && c.1 >= 0 && c.0 < (LAND_WIDTH_X as isize) && c.1 < (LAND_WIDTH_Y as isize)
}

#[derive(Debug)]
pub enum Cardinal {
    North,
    East,
    South,
    West,
}

pub struct Land {
    pub plots: Option<[[Plot; LAND_WIDTH_X]; LAND_WIDTH_Y]>,
}

impl Land {
    pub fn new() -> Land {
        Land { plots: None }
    }

    pub fn init_plots(&mut self) {
        self.plots = Some(Default::default());

        if let Some(ref mut plots) = self.plots {
            for i in 0..LAND_WIDTH_X {
                for j in 0..LAND_WIDTH_Y {
                    let cabbage_rng = &mut rand::thread_rng();
                    let enemy_rng = &mut rand::thread_rng();

                    let zeni_chance = cabbage_rng.gen_range(0.0, 1.0);
                    if zeni_chance < ZENI_GEN_CHANCE {
                        plots[i][j].dosh = (zeni_chance * 10.0) as u64
                    }

                    let enemy_chance = enemy_rng.gen_range(0.0, 1.0);
                    if enemy_chance < ENEMY_GEN_CHANCE {
                        plots[i][j].enemy = Some(Box::new(Bureaucrat {
                            dough: (enemy_chance * 50.0) as u64,
                            jurisdiction: Jurisdiction::Islands,
                        }));
                    }
                }
            }
        }
    }
}

#[derive(Copy, Clone, Debug, Deserialize)]
pub enum Jurisdiction {
    Mountains,
    Islands,
    Tundra,
    Desert,
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

#[derive(Default, Debug)]
pub struct Plot {
    pub dosh: u64,
    pub enemy: Option<Box<Intimidating>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coord_bounds() {
        let c1 = (0, 0);
        let c2 = (0, LAND_WIDTH_Y as isize - 1);
        let c3 = (LAND_WIDTH_X as isize - 1, 0);
        let c4 = (LAND_WIDTH_X as isize - 1, LAND_WIDTH_Y as isize - 1);
        let c5 = (0, LAND_WIDTH_Y as isize);
        let c6 = (LAND_WIDTH_X as isize, 0);
        let c7 = (LAND_WIDTH_X as isize, LAND_WIDTH_Y as isize);
        let c8 = (-1, 0);
        let c9 = (0, -1);

        assert_eq!(in_bounds(c1), true);
        assert_eq!(in_bounds(c2), true);
        assert_eq!(in_bounds(c3), true);
        assert_eq!(in_bounds(c4), true);

        assert_eq!(in_bounds(c5), false);
        assert_eq!(in_bounds(c6), false);
        assert_eq!(in_bounds(c7), false);
        assert_eq!(in_bounds(c8), false);
        assert_eq!(in_bounds(c9), false);
    }
}
