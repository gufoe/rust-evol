use std::{collections::HashMap, hash::Hash};

use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct Score {
    avg: f32,
    tot: f32,
    count: usize,
}
impl Score {
    pub fn record(&mut self, score: f32) {
        self.tot += score;
        self.count += 1;
        self.avg = self.tot / self.count as f32;
    }
}

pub type AlleleID = usize;
pub trait Allele {
    fn get_allele_id(&self) -> AlleleID;
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct GenePool<G: Hash + Eq + Serialize + Clone, A: Allele + Serialize + Clone> {
    pub genes: HashMap<G, HashMap<AlleleID, (A, Score)>>,
}

impl<G: Hash + Eq + Serialize + Clone, A: Allele + Serialize + Clone> GenePool<G, A> {
    pub fn record(&mut self, gene: &G, allele: A, fitness: f32) {
        if !self.genes.contains_key(&gene) {
            self.genes.insert(gene.clone(), HashMap::new());
        }
        let pool = self.genes.get_mut(gene).unwrap();
        let allele_id = allele.get_allele_id();
        if !pool.contains_key(&allele_id) {
            pool.insert(allele_id, (allele, Default::default()));
        }

        let (_, score) = pool.get_mut(&allele_id).unwrap();
        score.record(fitness);
    }
}
