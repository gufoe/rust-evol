use std::{collections::HashMap, hash::Hash};

use rand::random;
use serde::{Deserialize, Serialize};

use crate::{
    actions::Action,
    genome::{Genome, HasGenome},
    input::Sensor,
};

type NeuronID = String;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Neuron {
    pub charge: f32,
}
impl Default for Neuron {
    fn default() -> Neuron {
        Self { charge: 0.0 }
    }
}

impl Neuron {
    pub fn discharge(&mut self) {
        self.charge *= 0.5;
    }
    pub fn output(&self) -> f32 {
        self.charge.tanh()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub enum NeuralTarget {
    Neuron(NeuronID),
    Action(Action),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NeuralLink {
    pub inverse: bool,
    pub weight: f32,
}

impl NeuralLink {
    fn new() -> Self {
        Self {
            weight: rand::random::<f32>() * 4.0,
            inverse: random(),
        }
    }
    fn output(&self, mut x: f32) -> f32 {
        if self.inverse {
            x = 1.0 - x;
        }
        (x * self.weight).tan()
    }
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct NetNodes {
    pub hidden: HashMap<NeuronID, Neuron>,
    pub output: HashMap<Action, Neuron>,
}
impl NetNodes {
    pub fn create_link_node(&mut self, target: &NeuralTarget) {
        match target {
            NeuralTarget::Action(action) => {
                if !self.output.contains_key(action) {
                    self.output.insert(action.clone(), Neuron::default());
                }
            }
            NeuralTarget::Neuron(action) => {
                if !self.hidden.contains_key(action) {
                    self.hidden.insert(action.clone(), Neuron::default());
                }
            }
        }
    }
}

fn rand_h() -> NeuronID {
    format!("h-{}", random::<usize>() % 5)
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Net {
    // pub input_neurons: HashMap<Sensor, Neuron>,
    pub input_links: HashMap<Sensor, HashMap<NeuralTarget, NeuralLink>>,

    pub hidden_links: HashMap<NeuronID, HashMap<NeuralTarget, NeuralLink>>,
    pub nodes: NetNodes,
    #[serde(default)]
    pub color: [f32; 3],
}

impl Net {
    pub fn update_nodes(&mut self) {
        self.nodes = NetNodes::default();
        self.color = [0.0, 0.0, 0.0];
        let mut count = 0;
        let mut c0 = 0.0;
        let mut c1 = 0.0;
        for links in self.input_links.values() {
            for (target, link) in links {
                // self.color[count % self.color.len()]+= link.weight;
                count += 1;
                self.color[0]+= if link.inverse { 0.0 } else { 1.0 };
                self.color[2]+= if link.weight > 0.0 { 1.0 } else { 0.0 };
                c0 += 1.0;
                self.nodes.create_link_node(&target);
            }
        }
        for links in self.hidden_links.values() {
            for (target, link) in links {
                // self.color[count % self.color.len()]+= link.weight;
                count += 1;
                self.color[1]+= if link.inverse { 0.0 } else { 1.0 };
                self.color[2]+= if link.weight > 0.0 { 1.0 } else { 0.0 };
                c1 += 1.0;
                self.nodes.create_link_node(&target);
            }
        }
        self.color[0]/= c0 as f32;
        self.color[1]/= c1 as f32;
        self.color[2]/= count as f32;
        // self.color[0] = c0 / (c1 + c0);
        // self.color[1] = 0.1 +  (self.nodes.hidden.len() % 3) as f32 / 3.0;
        // self.color[2] = 0.1 +  (self.nodes.output.len() % 3) as f32 / 3.0;
        // self.color[2]/= c0 + c1;
        // self.color[0] = c0 as f32 / 5.0;
        // self.color[1] = c1 as f32 / 5.0;
        // self.color[2] = self.nodes.hidden.len() as f32 / 3.0;
    }
    pub fn clear(&mut self) {
        let clone: Vec<_> = self.hidden_links.keys().cloned().collect();
        for nid in &clone {
            if !self.nodes.hidden.contains_key(nid) {
                self.hidden_links.remove(nid);
            }
        }
    }
    pub fn randomize(&mut self) {
        for _ in 0..1 {
            self.add_link_from_sensor(random(), NeuralTarget::Action(random()));
        }
        for _ in 0..1 {
            self.add_link_from_sensor(random(), NeuralTarget::Neuron(rand_h()));
        }
        for _ in 0..1 {
            self.add_hidden_link(rand_h(), random());
        }
        for _ in 0..(random::<usize>() % 1) {
            if self.hidden_links.is_empty() {
                continue;
            }
            let keys: Vec<_> = self.hidden_links.keys().collect();
            let i = random::<usize>() % keys.len();
            let key = keys[i].clone();
            self.hidden_links.remove(&key).unwrap();
        }
        self.update_nodes();
        self.clear();
        print!("\r{:?}", self.nodes.hidden.len());
    }
    pub fn add_link_from_sensor(&mut self, source: Sensor, target: NeuralTarget) {
        let link = NeuralLink::new();
        match self.input_links.get_mut(&source) {
            Some(links) => {
                links.insert(target, link);
            }
            None => {
                let mut links = HashMap::new();
                links.insert(target, link);
                self.input_links.insert(source, links);
            }
        };
    }
    pub fn add_hidden_link(&mut self, source: String, action: Action) {
        let link = NeuralLink::new();
        let target = NeuralTarget::Action(action);
        match self.hidden_links.get_mut(&source) {
            Some(links) => {
                links.insert(target, link);
            }
            None => {
                let mut links = HashMap::new();
                links.insert(target, link);
                self.hidden_links.insert(source, links);
            }
        };
    }
    pub fn tick(&mut self, input: HashMap<Sensor, f32>) {
        let clone = self.clone();
        self.discharge();
        input.iter().for_each(|(sensor, x)| {
            let links = clone.input_links.get(sensor).unwrap();
            self.apply_synapse(*x, links);
        });
        clone.hidden_links.iter().for_each(|(source, links)| {
            if let Some(x) = clone.nodes.hidden.get(source) {
                let output = x.output();
                self.apply_synapse(output, links);
            }
        });
    }
    fn apply_synapse(&mut self, x: f32, links: &HashMap<NeuralTarget, NeuralLink>) {
        links.iter().for_each(|(target, link)| {
            let y = link.output(x);
            let neuron = match &target {
                NeuralTarget::Action(action) => self.nodes.output.get_mut(action),
                NeuralTarget::Neuron(neuron_id) => self.nodes.hidden.get_mut(neuron_id),
            };
            if let Some(neuron) = neuron {
                neuron.charge += y;
            }
        });
    }
    fn discharge(&mut self) {
        self.nodes.hidden.values_mut().for_each(|neuron| {
            neuron.discharge();
        });
        self.nodes.output.values_mut().for_each(|neuron| {
            neuron.discharge();
        });
    }
}

#[derive(Default, Clone)]
pub struct NetGenome {
    pub input_links: HashMap<Sensor, HashMap<NeuralTarget, NeuralLink>>,
    pub hidden_links: HashMap<NeuronID, HashMap<NeuralTarget, NeuralLink>>,
}

impl HasGenome<NetGenome> for Net {
    fn to_genome(&self) -> NetGenome {
        NetGenome {
            input_links: self.input_links.clone(),
            hidden_links: self.hidden_links.clone(),
        }
    }

    fn from_genome(genome: &NetGenome) -> Self {
        let mut ret = Net {
            input_links: genome.input_links.clone(),
            hidden_links: genome.hidden_links.clone(),
            ..Default::default()
        };
        ret.update_nodes();
        ret
    }
}
impl Genome for NetGenome {
    fn mix(&self, p2: &NetGenome) -> NetGenome {
        let mut ret = self.clone();
        // return ret;
        for (i, links) in &p2.input_links {
            for (target, link) in links {
                if random() {
                    if !ret.input_links.contains_key(i) {
                        ret.input_links.insert(*i, HashMap::new());
                    }
                    ret.input_links
                        .get_mut(i)
                        .unwrap()
                        .insert(target.clone(), link.clone());
                } else if ret.input_links.contains_key(i) && random() {
                    ret.input_links.remove(i);
                }
            }
        }
        for (i, links) in &p2.hidden_links {
            for (target, link) in links {
                if random() {
                    if !ret.hidden_links.contains_key(i) {
                        ret.hidden_links.insert(i.clone(), HashMap::new());
                    }
                    ret.hidden_links
                        .get_mut(i)
                        .unwrap()
                        .insert(target.clone(), link.clone());
                } else if ret.hidden_links.contains_key(i) && random() {
                    ret.hidden_links.remove(i);
                }
            }
        }
        ret
    }
}
