// TODO refactor game logic out of this module.
use rand::Rng;
use std::collections::HashMap;
use std::str::FromStr;

use creatures::{entity, Bureaucrat, Intimidating, UberDriver};
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
    // TODO Remove as options, remove init_plots and merge into new.
    pub plots: Option<[[Plot; LAND_WIDTH_X]; LAND_WIDTH_Y]>,
    pub entity_locations: Option<HashMap<&'static str, Vec<Coord>>>,
}

impl Land {
    pub fn new() -> Land {
        Land {
            plots: None,
            entity_locations: None,
        }
    }

    pub fn init_plots(&mut self) {
        let mut loc_map = HashMap::new();
        loc_map.insert(entity::TAXI, Vec::new());
        loc_map.insert(entity::ENEMY, Vec::new());

        self.plots = Some(Default::default());

        if let Some(ref mut plots) = self.plots {
            let rng = &mut rand::thread_rng();

            for i in 0..LAND_WIDTH_X {
                for j in 0..LAND_WIDTH_Y {
                    let zeni_chance = rng.gen_range(0.0, 1.0);
                    if zeni_chance < ZENI_GEN_CHANCE {
                        plots[i][j].dosh = (zeni_chance * 10.0) as u64;
                    }

                    let enemy_chance = rng.gen_range(0.0, 1.0);
                    if enemy_chance < ENEMY_GEN_CHANCE {
                        plots[i][j].enemy = Some(Box::new(Bureaucrat {
                            dough: (enemy_chance * 50.0) as u64,
                            jurisdiction: Jurisdiction::Islands,
                        }));
                        loc_map.get_mut(entity::ENEMY).unwrap().push((i, j));
                    }
                }
            }

            let uber_x = rng.gen_range(0, LAND_WIDTH_X);
            let uber_y = rng.gen_range(0, LAND_WIDTH_Y);
            plots[uber_x][uber_y].driver = Some(Default::default());
            loc_map
                .get_mut(entity::TAXI)
                .unwrap()
                .push((uber_x, uber_y));
        };

        self.entity_locations = Some(loc_map);
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
    pub driver: Option<UberDriver>,
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
