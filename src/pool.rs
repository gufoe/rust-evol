use std::{cmp::Ordering, collections::HashMap, hash::Hash};

use rand::random;
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

pub type AlleleID = u64;
pub trait Allele<G: Hash + Eq + Serialize + Clone> {
    fn get_allele_id(&self) -> AlleleID;
    fn get_gene_requirements(&self) -> Vec<G>;
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct GenePool<G: Hash + Eq + Serialize + Clone, A: Allele<G> + Serialize + Clone> {
    pub genes: HashMap<G, HashMap<AlleleID, (A, Score)>>,
}

impl<G: Hash + Eq + Serialize + Clone, A: Allele<G> + Serialize + Clone> GenePool<G, A> {
    pub fn new() -> Self {
        Self {
            genes: HashMap::new(),
        }
    }
    pub fn record(&mut self, gene: &G, allele: &A, fitness: f32) {
        if !self.genes.contains_key(&gene) {
            self.genes.insert(gene.clone(), HashMap::new());
        }
        let pool = self.genes.get_mut(gene).unwrap();
        let allele_id = allele.get_allele_id();
        if !pool.contains_key(&allele_id) {
            pool.insert(allele_id, (allele.clone(), Default::default()));
        }

        let (_, score) = pool.get_mut(&allele_id).unwrap();
        score.record(fitness);

        // if pool.len() > 10 {
        //     let min_use = 1000;
        //     let values = pool
        //         .values()
        //         .filter_map(|(_, s)| if s.count < min_use { None } else { Some(s.avg) })
        //         .collect::<Vec<_>>();
        //     let count = values.len();
        //     if count > 0 {
        //         let avg = values.iter().sum::<f32>() / count as f32;
        //         pool.retain(|_, (_, s)| s.count < min_use || s.avg >= avg);
        //     }
        // }
    }
    pub fn get_genes(&self) -> Vec<G> {
        self.genes.keys().cloned().collect()
    }
    pub fn build(&self, genes: Vec<G>) -> HashMap<G, A> {
        let mut ret = HashMap::new();
        for gene in genes {
            self.require(&gene, &mut ret);
        }
        ret
    }
    pub fn prune(&mut self) {
        // let dipendenze = HashMap::new();

        // self.genes.values().for_each(|alleles| {
        //     alleles.values().for_each(|(allele, _)| {
        //         allele.get_gene_requirements().iter().for_each(|req| {
        //             req
        //         });
        //     });
        // });

        self.genes.iter_mut().for_each(|(_, alleles)| {
            if alleles.len() < 3 {
                return;
            }
            let avg: f32 = alleles.values().map(|(_, s)| s.avg).sum::<f32>() / alleles.len() as f32;
            let indexes = alleles
                .values()
                .filter_map(|(a, s)| {
                    if s.avg < avg {
                        Some(a.get_allele_id())
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();
            indexes.iter().for_each(|id| {
                alleles.remove(&id);
            });
            // alleles.iter().for_each()
        });
    }
    fn require(&self, gene: &G, ret: &mut HashMap<G, A>) {
        if ret.contains_key(gene) {
            return;
        }
        let alleles = self.genes.get(gene);
        if alleles.is_none() {
            return;
        }
        let alleles = alleles.unwrap();

        if alleles.is_empty() {
            return;
        }

        let allele = alleles
            .values()
            .max_by(|(_a, a_score), (_b, b_score)| {
                if a_score.avg > b_score.avg {
                    Ordering::Greater
                } else {
                    Ordering::Less
                }
            })
            .unwrap()
            .0
            .clone();
        ret.insert(gene.clone(), allele.clone());
        for req in allele.get_gene_requirements() {
            self.require(&req, ret)
        }
    }
}
