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
        if self.time > 100 {
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
        let mut survivors: Vec<NetGenome> = self
            .sim
            .replicants
            .iter()
            .filter(|rep| rep.is_alive(&self.sim.world, &self.sim.mapper))
            .map(|rep| rep.to_genome())
            .collect();
        // println!("After self.sim: {:#?}", survivors);

        print!(
            "gen {} survivors: {}/{}\n",
            self.generation,
            survivors.len(),
            self.sim.replicants.len()
        );
        if survivors.is_empty() {
            survivors = self
                .sim
                .replicants
                .iter()
                .map(|rep| rep.to_genome())
                .collect();
        }
        self.sim.replicants.clear();

        let survc = survivors.len();
        let mut pools = [vec![], vec![], vec![]];
        while let Some(genome) = survivors.pop() {
            let c = genome.color;
            let max = c[0].max(c[1]).max(c[2]);
            if c[0] == max {
                pools[0].push(genome);
            } else if c[1] == max {
                pools[1].push(genome);
            } else {
                pools[2].push(genome);
            }
        }
        println!("{} {} {}", pools[0].len(), pools[1].len(), pools[2].len());

        for _ in 0..self.pop_size {
            let tot = survc as f32;
            let a = pools[0].len() as f32;
            let b = pools[1].len() as f32;
            let x = if random::<f32>() < a / tot {
                0
            } else if random::<f32>() < b / (tot - a) {
                1
            } else {
                2
            };
            let pool = &mut pools[x];
            // let i = i % pool.len();
            let i = random::<usize>() % pool.len();
            let parent_a = pool.get(i).unwrap();
            let i = random::<usize>() % pool.len();
            let parent_b = pool.get(i).unwrap();

            let mut child = Replicant::from_genome(&parent_a.mix(&parent_b));
            if random::<f32>() > 0.99 {
                child.net.randomize();
            }
            // if i == 0 {
            //     parent_a.net.randomize();
            // }
            self.sim.replicants.push(child);
        }
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
