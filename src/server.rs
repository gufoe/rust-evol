use std::{collections::HashMap, path::PathBuf, thread};

use rand::random;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use serde::{Deserialize, Serialize};

use crate::{
    genome::{Genome, HasGenome},
    net::{NetGenome, NeuralNode, NeuralSource, NeuralTarget},
    pool::GenePool,
    replicant::Replicant,
    simulation::Simulation,
};

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct Server {
    pub auto_save: Option<PathBuf>,
    pub generation: usize,
    pub time: usize,
    pub sim: Simulation,
    pub gene_pools: HashMap<usize, GenePool<NeuralTarget, NeuralNode>>,
    pub pop_size: usize,
    pub prev_survival: [usize; 3],
}
impl Server {
    pub fn setup(&mut self) {
        self.sim.world.width = 80;
        self.sim.world.height = 80;
        self.sim.mapper.clip = Some((self.sim.world.width, self.sim.world.height));
        self.sim.mapper.clip = None;
        // self.sim.world.lifespan = 100;
        self.pop_size = 3000;
        // eprintln!("[server] init {}", self.generation);
        if self.generation == 0 {
            for _ in 0..self.pop_size {
                let mut genome = NetGenome::default();
                genome.randomize();
                genome.randomize_color();
                let rep = Replicant::from_genome(&genome);
                self.sim.replicants.push(rep);
            }
        }
        self.sim.setup();
    }

    pub fn tick(&mut self) {
        if self.time > 300 {
            //} + crate::rng::random::<usize>(self.generation as u64) % 50 {
            // if self.time > self.sim.world.lifespan {
            // println!("Round ended");
            if let Some(path) = self.auto_save.clone() {
                let clone = self.clone();
                thread::spawn(move || {
                    let tmp_file = format!("{}-tmp", &path.to_string_lossy());
                    let tmp_file_json = format!("{}.repl.json", &path.to_string_lossy());
                    // let tmp_file_json_pool = format!("{}.pool.json", &path.to_string_lossy());
                    let ser = bincode::serialize(&clone).unwrap();
                    std::fs::write(&tmp_file, &ser).unwrap();
                    std::fs::rename(&tmp_file, &path).unwrap();
                    let ser =
                        serde_json::to_string_pretty(&clone.sim.replicants[0].to_genome().value())
                            .unwrap();
                    // let pool = serde_json::to_string_pretty(&clone.gene_pools).unwrap();
                    std::fs::write(&tmp_file_json, &ser).unwrap();
                    // std::fs::write(&tmp_file_json_pool, &pool).unwrap();
                });
            }
            self.finish_round();
            self.time = 0;
            self.generation += 1;
        }

        if self.time == 0 {
            self.setup();
        }

        assert!(self.sim.replicants.len() == self.pop_size);

        // println!("Round {}", self.time);
        // Normal cycle
        self.sim.tick();
        self.time += 1;
    }

    fn _get_alive_dead(&self) -> HashMap<usize, (Vec<usize>, Vec<usize>)> {
        let mut ret = HashMap::new();

        self.sim
            .replicants
            .iter()
            .skip(random::<usize>() % self.sim.replicants.len())
            .enumerate()
            .for_each(|(i, rep)| {
                let pool = rep.net.pool();
                if !ret.contains_key(&pool) {
                    ret.insert(pool, (vec![], vec![]));
                }
                if rep.is_alive(&self.sim.world, &self.sim.mapper) {
                    ret.get_mut(&pool).unwrap().0.push(i);
                } else {
                    ret.get_mut(&pool).unwrap().1.push(i);
                }
            });
        ret
    }
    fn get_pools(&self) -> HashMap<usize, Vec<Replicant>> {
        let mut ret = HashMap::new();
        let mut survivors: Vec<&Replicant> = self
            .sim
            .replicants
            .par_iter()
            .filter(|rep| rep.is_alive(&self.sim.world, &self.sim.mapper))
            .collect();

        // let avg_moves = 2 * survivors.iter().map(|a| a.moves).sum::<usize>() / survivors.len();

        // let mut survivors: Vec<&Replicant> = survivors
        //     .par_iter()
        //     .filter(|x| x.moves <= avg_moves)
        //     .map(|r| r.clone())
        //     .collect();

        // println!("After self.sim: {:#?}", survivors);

        if !ret.contains_key(&0) {
            ret.insert(0, vec![]);
        }
        if !ret.contains_key(&1) {
            ret.insert(1, vec![]);
        }
        if !ret.contains_key(&2) {
            ret.insert(2, vec![]);
        }
        while let Some(genome) = survivors.pop() {
            let pool = genome.net.pool();
            ret.get_mut(&pool).unwrap().push(genome.clone());
        }
        ret
    }

    fn score_genes(&mut self) {
        let pools = self.get_pools();
        let pools = [
            (pools.get(&0).unwrap().len() * pools.len()) as f32 / self.pop_size as f32,
            (pools.get(&1).unwrap().len() * pools.len()) as f32 / self.pop_size as f32,
            (pools.get(&2).unwrap().len() * pools.len()) as f32 / self.pop_size as f32,
        ];
        for rep in &self.sim.replicants {
            let pool = rep.net.pool();
            let score = if rep.is_alive(&self.sim.world, &self.sim.mapper) {
                1.0 + pools[pool]
            } else {
                0.0
            };
            if !self.gene_pools.contains_key(&pool) {
                self.gene_pools.insert(pool, GenePool::new());
            }
            let pool = self.gene_pools.get_mut(&pool).unwrap();

            for (source, node) in &rep.net.nodes {
                pool.record(source, node, score)
            }
        }
        self.gene_pools.values_mut().for_each(|pool| {
            // pool.prune();
        })
    }

    fn finish_round(&mut self) {
        self.score_genes();
        self.print_pools_stats();
        self.replace_replicants_v2();
    }
    fn print_pools_stats(&mut self) {
        let pools = self.get_pools();
        println!(
            "{:.3} {:.3} {:.3}",
            (pools.get(&0).unwrap().len() * pools.len()) as f32 / self.pop_size as f32,
            (pools.get(&1).unwrap().len() * pools.len()) as f32 / self.pop_size as f32,
            (pools.get(&2).unwrap().len() * pools.len()) as f32 / self.pop_size as f32,
        );
    }
    fn replace_replicants_v2(&mut self) {
        self.sim.replicants.clear();
        for (pool_i, pool) in &self.gene_pools {
            let mut inserted = 0;
            while inserted < self.pop_size / self.gene_pools.len() {
                let mut genome = NetGenome::default();
                let alleles = pool.build(
                    pool.get_genes()
                        .into_iter()
                        .filter(|gene| match gene {
                            NeuralTarget::Action(_) => true,
                            _ => false,
                        })
                        .collect(),
                );
                genome.nodes = alleles;
                genome.color = [0.0, 0.0, 0.0];
                *genome.color.get_mut(*pool_i).unwrap() = 1.0;
                let pmut = if *pool_i == 0 {
                    0.99
                } else if *pool_i == 1 {
                    0.995
                } else {
                    0.999
                };
                for _ in 0..200 {
                    if random::<f32>() > pmut {
                        genome.randomize();
                    }
                    let child = Replicant::from_genome(&genome);
                    self.sim.replicants.push(child);
                    inserted += 1;
                }
            }
        }
    }
    fn _replace_replicants(&mut self) {
        // println!("Replicants: {:#?}", self.sim.replicants);
        let pools = self.get_pools();

        let mut new_reps: Vec<Replicant> = vec![];
        for i in 0..self.pop_size {
            let x = i % pools.len();

            let pool = pools.get(&x).unwrap();

            if !pool.is_empty() {
                let parent_ai = random::<usize>() % pool.len();
                let parent_a = pool.get(parent_ai).unwrap();
                let mut genome = if random::<f32>() > 0.9 {
                    // let parent_b = pool.get(&random() % pool.len()).unwrap();
                    let mut parent_b = pool.get(0).unwrap();
                    for (parent_bi, rep) in pool.iter().enumerate() {
                        if parent_bi == parent_ai
                            || (parent_bi != parent_ai
                                && parent_a.dist(parent_b) > parent_a.dist(rep))
                        {
                            parent_b = rep;
                        }
                    }
                    parent_a.to_genome().mix(&parent_b.to_genome())
                } else {
                    parent_a.to_genome()
                };

                let pmut = if x == 0 {
                    0.9
                } else if x == 1 {
                    0.95
                } else {
                    0.99
                };
                if random::<f32>() > pmut {
                    genome.randomize();
                }
                new_reps.push(Replicant::from_genome(&genome));
            } else {
                let mut rep = Replicant::default();
                // rep.net.randomize();
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
