use std::path::PathBuf;

use rand::random;
use serde::{Deserialize, Serialize};

use crate::{
    genome::{Genome, HasGenome},
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
            if let Some(path) = &self.auto_save {
                let tmp_file = format!("{}-tmp", &path.to_string_lossy());
                let ser = bincode::serialize(self).unwrap();
                std::fs::write(&tmp_file, &ser).unwrap();
                std::fs::rename(&tmp_file, &path).unwrap();
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
        let mut survivors: Vec<_> = self
            .sim
            .replicants
            .iter()
            .cloned()
            .filter(|rep| {
                rep.is_alive(&self.sim.world)
            })
            .collect();
        // println!("After self.sim: {:#?}", survivors);

        print!(
            "gen {} survivors: {}/{}\n",
            self.generation,
            survivors.len(),
            self.sim.replicants.len()
        );
        if survivors.is_empty() {
            survivors = self.sim.replicants.clone();
        }
        self.sim.replicants.clear();
        let len = survivors.len();
        for _ in 0..self.pop_size {
            // println!("Filling");
            let i = random::<usize>() % len;
            let parent_a = survivors.get(i).unwrap();
            let i = random::<usize>() % len;
            let parent_b = survivors.get(i).unwrap();

            let mut child =
                Replicant::from_genome(&parent_a.to_genome().mix(&parent_b.to_genome()));
            if random::<f32>() > 0.9999 {
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
