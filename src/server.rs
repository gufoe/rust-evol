use std::{path::PathBuf, thread};

use rand::random;
use serde::{Deserialize, Serialize};

use crate::{
    genome::{Genome, HasGenome},
    net::NetGenome,
    replicant::Replicant,
    simulation::Simulation,
};

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct Server {
    pub auto_save: Option<PathBuf>,
    pub generation: usize,
    pub time: usize,
    pub sim: Simulation,
    pub pop_size: usize,
    pub prev_survival: [usize; 3],
}
impl Server {
    pub fn setup(&mut self) {
        self.sim.world.width = 128;
        self.sim.world.height = 128;
        // self.sim.world.lifespan = 100;
        self.pop_size = 3000;
        for _ in 0..self.pop_size {
            let mut rep = Replicant::default();
            rep.net.randomize();
            self.sim.replicants.push(rep);
        }
    }

    pub fn tick(&mut self) {
        if self.time > 200 + crate::rng::random::<usize>(self.generation as u64) % 120 {
            // if self.time > self.sim.world.lifespan {
            // println!("Round ended");
            self.finish_round();
            self.time = 0;
            self.generation += 1;
            if let Some(path) = self.auto_save.clone() {
                let clone = self.clone();
                thread::spawn(move || {
                    let tmp_file = format!("{}-tmp", &path.to_string_lossy());
                    let ser = bincode::serialize(&clone).unwrap();
                    std::fs::write(&tmp_file, &ser).unwrap();
                    std::fs::rename(&tmp_file, &path).unwrap();
                });
            }
        }

        if self.time == 0 {
            self.sim.setup();
        }

        // println!("Round {}", self.time);
        // Normal cycle
        self.sim.tick();
        self.time += 1;
    }

    fn finish_round(&mut self) {
        // println!("Replicants: {:#?}", self.sim.replicants);
        let mut survivors: Vec<Replicant> = self
            .sim
            .replicants
            .iter()
            .filter(|rep| rep.is_alive(&self.sim.world, &self.sim.mapper))
            .cloned()
            .collect();
        // println!("After self.sim: {:#?}", survivors);

        let mut pools = [vec![], vec![], vec![]];
        while let Some(genome) = survivors.pop() {
            let pool = genome.net.pool();
            pools[pool].push(genome);
        }

        println!(
            "{:.3} {:.3} {:.3}",
            (pools[0].len() * pools.len()) as f32 / self.pop_size as f32,
            (pools[1].len() * pools.len()) as f32 / self.pop_size as f32,
            (pools[2].len() * pools.len()) as f32 / self.pop_size as f32,
        );
        let mut new_reps: Vec<Replicant> = vec![];
        for i in 0..self.pop_size {
            let x = i % pools.len();

            let pool = pools.get(x).unwrap();

            if !pool.is_empty() {
                let i = random::<usize>() % pool.len();
                let parent_a = pool.get(i).unwrap().clone();
                let mut parent_b = pool.get(0).unwrap();
                for rep in pool {
                    if parent_a.dist(parent_b) > parent_a.dist(rep) {
                        parent_b = rep;
                    }
                }
                let pmut = if x == 0 {
                    0.95
                } else if x == 1 {
                    0.99
                } else {
                    0.999
                };
                let mut child = if random::<f32>() > 0.5 {
                    Replicant::from_genome(&parent_a.to_genome().mix(&parent_b.to_genome()))
                } else {
                    parent_a
                };
                if random::<f32>() > pmut {
                    child.net.randomize();
                }
                new_reps.push(child);
            } else {
                let mut rep = Replicant::default();
                rep.net.randomize();
                rep.net.color[x] = 1.0;
                new_reps.push(rep);
            }

            // let i = i % pool.len();

            // if i == 0 {
            //     parent_a.net.randomize();
            // }
        }
        self.sim.replicants = new_reps;
    }
}

// fn setup(sim: &mut Simulation, pop_size: usize) {
//     for _ in 0..pop_size {
//         let mut rep = Replicant::default();
//         rep.state.x = 90;
//         rep.initialize();
//         sim.replicants.push(rep);
//     }
// }
