use std::collections::{HashMap, HashSet};

use rand::random;
use serde::{Deserialize, Serialize};

use crate::{replicant::Replicant, world::World};

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Simulation {
    pub world: World,
    pub replicants: Vec<Replicant>,
    // #[serde(skip_serializing)]
    mapper: CellMapper,
}

impl Simulation {
    pub fn setup(&mut self) {
        self.mapper = CellMapper::default();
        self.replicants.iter_mut().for_each(|rep| {
            while !self.mapper.move_abs(
                &mut rep.pos,
                (
                    random::<i32>().abs() % self.world.width,
                    random::<i32>().abs() % self.world.height,
                ),
            ) {}
        });
    }

    pub fn tick(&mut self) {
        use rayon::prelude::*;
        self.replicants.par_iter_mut().for_each(|rep| {
            let mut input = HashMap::new();
            rep.net.input_links.keys().for_each(|sensor| match sensor {
                crate::input::Sensor::LocX => {
                    input.insert(
                        sensor.clone(),
                        (rep.pos.0 as f32) / (self.world.width as f32) - 0.5,
                    );
                }
                crate::input::Sensor::LocY => {
                    input.insert(
                        sensor.clone(),
                        (rep.pos.1 as f32) / (self.world.height as f32) - 0.5,
                    );
                }
                crate::input::Sensor::Osc => {
                    input.insert(sensor.clone(), (rep.time as f32 / 30.0).sin());
                }
                crate::input::Sensor::Bias(x) => {
                    input.insert(sensor.clone(), (x / i8::MAX).into());
                }
                crate::input::Sensor::Random => {
                    input.insert(sensor.clone(), random());
                }
            });
            rep.net.tick(input);
            rep.time += 1;
        });
        self.replicants.iter_mut().for_each(|rep| {
            rep.net.nodes.output.iter().for_each(|(action, neuron)| {
                let out = neuron.output();
                let fire = random::<f32>() < out.abs();
                if !fire {
                    return;
                }
                match action {
                    crate::actions::Action::MovX => {
                        self.mapper.move_rel(&mut rep.pos, (out.signum() as i32, 0))
                    }
                    crate::actions::Action::MovY => {
                        self.mapper.move_rel(&mut rep.pos, (0, out.signum() as i32))
                    }
                };
                if rep.pos.0 >= self.world.width {
                    rep.pos.0 = self.world.width - 1
                } else if rep.pos.0 < 0 {
                    rep.pos.0 = 0
                }
                if rep.pos.1 >= self.world.height {
                    rep.pos.1 = self.world.height - 1
                } else if rep.pos.1 < 0 {
                    rep.pos.1 = 0
                }
            });
        });
    }
}

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
struct CellMapper {
    filled_cells: HashSet<(i32, i32)>,
}
impl CellMapper {
    pub fn move_rel(&mut self, current_pos: &mut (i32, i32), position: (i32, i32)) -> bool {
        let final_pos = (current_pos.0 + position.0, current_pos.1 + position.1);
        self.move_abs(current_pos, final_pos)
    }
    pub fn move_abs(&mut self, current_pos: &mut (i32, i32), final_pos: (i32, i32)) -> bool {
        if self.filled_cells.contains(&final_pos) {
            false
        } else {
            self.filled_cells.remove(&current_pos);
            self.filled_cells.insert(final_pos);
            current_pos.0 = final_pos.0;
            current_pos.1 = final_pos.1;
            true
        }
    }
}
