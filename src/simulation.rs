use std::collections::{HashMap, HashSet};

use rand::random;
use serde::{Deserialize, Serialize};

use crate::{input::NeighbourType, replicant::Replicant, world::World};

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Simulation {
    pub world: World,
    pub replicants: Vec<Replicant>,
    // #[serde(skip_serializing)]
    pub mapper: CellMapper,
}

impl Simulation {
    pub fn setup(&mut self) {
        self.mapper.reset();
        self.replicants.iter_mut().for_each(|rep| {
            while !self.mapper.add_abs(
                &mut rep.pos,
                (
                    random::<i32>().abs() % self.world.width,
                    random::<i32>().abs() % self.world.height,
                ),
                rep.net.pool(),
            ) {}
        });
    }

    pub fn tick(&mut self) {
        use rayon::prelude::*;
        let actions = self
            .replicants
            .par_iter_mut()
            .enumerate()
            .map(|(rep_i, rep)| {
                let is_alive = rep.is_alive(&self.world, &self.mapper);
                let pool = rep.net.pool();
                rep.net
                    .sensors
                    .iter_mut()
                    .for_each(|(sensor, value)| match sensor {
                        crate::input::Sensor::Loc { x } => {
                            *value = if *x {
                                (rep.pos.0 as f32) / (self.world.width as f32) - 0.5
                            } else {
                                (rep.pos.1 as f32) / (self.world.height as f32) - 0.5
                            };
                        }
                        crate::input::Sensor::Osc(x) => {
                            *value = (rep.time as f32 / (x + 1) as f32).sin();
                        }
                        crate::input::Sensor::Bias(x) => {
                            *value = (x / i8::MAX).into();
                        }
                        crate::input::Sensor::Random => {
                            *value = random();
                        }
                        crate::input::Sensor::Alive => {
                            *value = is_alive as u8 as f32;
                        }
                        crate::input::Sensor::Neighbour { vert, incr, kind } => {
                            let mut check = rep.pos;
                            let p = if *vert { &mut check.1 } else { &mut check.0 };
                            *p += if *incr { 1 } else { -1 };

                            let ok = match kind {
                                NeighbourType::Any => self.mapper.has(check.0, check.1),
                                NeighbourType::Empty => !self.mapper.has(check.0, check.1),
                                NeighbourType::Pool(pool) => {
                                    self.mapper.is(check.0, check.1, *pool)
                                }
                                NeighbourType::Friend => self.mapper.is(check.0, check.1, pool),
                                NeighbourType::Enemy => !self.mapper.is(check.0, check.1, pool),
                            };
                            *value = if ok { 1.0 } else { 0.0 };
                        }
                    });
                rep.time += 1;
                (rep_i, rep.net.tick())
            })
            .collect::<Vec<_>>();
        actions.iter().for_each(|(rep_i, actions)| {
            let rep = self.replicants.get_mut(*rep_i).unwrap();
            actions.iter().for_each(|action| {
                match action {
                    crate::actions::Action::IncX => {
                        if self.mapper.clip.is_some() || rep.pos.0 + 1 < self.world.width {
                            self.mapper.move_rel(&mut rep.pos, (1, 0), rep.net.pool());
                            rep.moves += 1;
                        }
                    }
                    crate::actions::Action::IncY => {
                        if self.mapper.clip.is_some() || rep.pos.1 + 1 < self.world.height {
                            self.mapper.move_rel(&mut rep.pos, (0, 1), rep.net.pool());
                            rep.moves += 1;
                        }
                    }
                    crate::actions::Action::DecX => {
                        if self.mapper.clip.is_some() || rep.pos.0 - 1 > 0 {
                            self.mapper.move_rel(&mut rep.pos, (-1, 0), rep.net.pool());
                            rep.moves += 1;
                        }
                    }
                    crate::actions::Action::DecY => {
                        if self.mapper.clip.is_some() || rep.pos.1 - 1 > 0 {
                            self.mapper.move_rel(&mut rep.pos, (0, -1), rep.net.pool());
                            rep.moves += 1;
                        }
                    }
                };
            });
        });
    }
}

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct CellMapper {
    pub clip: Option<(i32, i32)>,
    filled_cells: HashMap<(i32, i32), usize>,
}
impl CellMapper {
    pub fn reset(&mut self) {
        self.filled_cells.clear();
    }
    pub fn normalize(&self, x: i32, y: i32) -> (i32, i32) {
        match self.clip {
            None => (x, y),
            Some((w, h)) => (
                if x >= w {
                    x - w
                } else if x < 0 {
                    x + w
                } else {
                    x
                },
                if y >= h {
                    y - h
                } else if y < 0 {
                    y + h
                } else {
                    y
                },
            ),
        }
    }
    pub fn has(&self, x: i32, y: i32) -> bool {
        let (x, y) = self.normalize(x, y);
        self.filled_cells.contains_key(&(x, y))
    }
    pub fn is(&self, x: i32, y: i32, pool: usize) -> bool {
        let (x, y) = self.normalize(x, y);
        let x = self.filled_cells.get(&(x, y));
        if x.is_none() {
            false
        } else {
            *x.unwrap() == pool
        }
    }
    pub fn isexcept(&self, x: i32, y: i32, pool: usize) -> bool {
        let (x, y) = self.normalize(x, y);
        let x = self.filled_cells.get(&(x, y));
        if x.is_none() {
            false
        } else {
            *x.unwrap() != pool
        }
    }
    pub fn move_rel(
        &mut self,
        current_pos: &mut (i32, i32),
        position: (i32, i32),
        pool: usize,
    ) -> bool {
        let final_pos = (current_pos.0 + position.0, current_pos.1 + position.1);
        self.move_abs(current_pos, final_pos, pool)
    }
    pub fn move_abs(
        &mut self,
        current_pos: &mut (i32, i32),
        final_pos: (i32, i32),
        pool: usize,
    ) -> bool {
        let final_pos = self.normalize(final_pos.0, final_pos.1);
        if self.filled_cells.contains_key(&final_pos) {
            false
        } else {
            self.filled_cells.remove(current_pos);
            self.filled_cells.insert(final_pos, pool);
            current_pos.0 = final_pos.0;
            current_pos.1 = final_pos.1;
            true
        }
    }
    pub fn add_abs(
        &mut self,
        current_pos: &mut (i32, i32),
        final_pos: (i32, i32),
        pool: usize,
    ) -> bool {
        let final_pos = self.normalize(final_pos.0, final_pos.1);
        if self.filled_cells.contains_key(&final_pos) {
            false
        } else {
            self.filled_cells.insert(final_pos, pool);
            current_pos.0 = final_pos.0;
            current_pos.1 = final_pos.1;
            true
        }
    }
}
