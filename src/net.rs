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
        self.charge *= 0.9;
    }
    pub fn output(&self) -> f32 {
        self.charge.tanh()
    }
    fn fires(&mut self) -> bool {
        let out = self.output();
        if out.abs() < 0.5 {
            return false;
        }
        let fire = random::<f32>() < out.abs();
        if !fire {
            return false;
        }
        self.charge = 0.0;
        true
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
    format!("h-{}", random::<usize>() % 10)
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Net {
    // pub input_neurons: HashMap<Sensor, Neuron>,
    pub input_links: HashMap<Sensor, HashMap<NeuralTarget, NeuralLink>>,

    pub hidden_links: HashMap<NeuronID, HashMap<NeuralTarget, NeuralLink>>,
    pub nodes: NetNodes,
    pub color: [f32; 3],
}

impl Default for Net {
    fn default() -> Self {
        let mut color = [0.0; 3];
        *color.get_mut(random::<usize>()%3).unwrap() = random::<f32>().abs();
        Self {
            color,
            input_links: Default::default(),
            hidden_links: Default::default(),
            nodes: Default::default(),
        }
    }
}
impl Net {
    pub fn pool(&self) -> usize {
        let c = self.color;
        let max = c[0].max(c[1]).max(c[2]);
        if c[0] == max {
            0
        } else if c[1] == max {
            1
        } else {
            2
        }
    }
    // pub fn links(
    //     &self,
    // ) -> std::iter::Chain<
    //     std::collections::hash_map::Values<
    //         '_,
    //         std::string::String,
    //         HashMap<NeuralTarget, NeuralLink>,
    //     >,
    //     std::collections::hash_map::Values<'_, Sensor, HashMap<NeuralTarget, NeuralLink>>,
    // > {
    //     self.hidden_links.values().chain(self.input_links.values())
    // }
    pub fn update_nodes(&mut self) {
        self.nodes = NetNodes::default();
        for links in self.input_links.values() {
            for (target, link) in links {
                self.nodes.create_link_node(&target);
            }
        }
        for links in self.hidden_links.values() {
            for (target, link) in links {
                self.nodes.create_link_node(&target);
            }
        }
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
        let i = random::<usize>() % self.color.len();
        let c = self.color.get_mut(i).unwrap();
        *c += (random::<f32>() * 2.0 - 1.0) * 0.3;
        if *c > 1.0 {
            *c = 1.0 - (*c - 1.0);
        }
        if *c < 0.0 {
            *c = c.abs();
        }
        *c = c.min(1.0).max(0.0);
        let max = self.color[0].max(self.color[1]).max(self.color[2]);
        for x in &mut self.color {
            if *x != max {
                *x -= 0.01;
                *x = x.min(1.0).max(0.0);
            }
        }

        if random() {
            for _ in 0..1 {
                self.add_link_from_sensor(random(), NeuralTarget::Action(random()));
            }
        }
        if random() {
            for _ in 0..1 {
                self.add_link_from_sensor(random(), NeuralTarget::Neuron(rand_h()));
            }
        }
        if random() {
            for _ in 0..1 {
                let h = rand_h();
                self.add_link_from_sensor(random(), NeuralTarget::Neuron(h.clone()));
                self.add_hidden_link(h.clone(), random());
            }
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
        for _ in 0..(random::<usize>() % 4) {
            if self.input_links.is_empty() {
                continue;
            }
            let keys: Vec<_> = self.input_links.keys().collect();
            let i = random::<usize>() % keys.len();
            let key = keys[i].clone();
            self.input_links.remove(&key).unwrap();
        }
        self.update_nodes();
        self.clear();
        // print!("\r{:?}", self.nodes.hidden.len());
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
    pub fn tick(&mut self, input: HashMap<Sensor, f32>) -> Vec<Action> {
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

        return self
            .nodes
            .output
            .iter_mut()
            .filter_map(|(k, n)| if n.fires() { Some(k) } else { None })
            .cloned()
            .collect();
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
    pub color: [f32; 3],
    pub input_links: HashMap<Sensor, HashMap<NeuralTarget, NeuralLink>>,
    pub hidden_links: HashMap<NeuronID, HashMap<NeuralTarget, NeuralLink>>,
}

impl HasGenome<NetGenome> for Net {
    fn to_genome(&self) -> NetGenome {
        NetGenome {
            color: self.color.clone(),
            input_links: self.input_links.clone(),
            hidden_links: self.hidden_links.clone(),
        }
    }

    fn from_genome(genome: &NetGenome) -> Self {
        let mut ret = Net {
            color: genome.color.clone(),
            input_links: genome.input_links.clone(),
            hidden_links: genome.hidden_links.clone(),
            ..Default::default()
        };
        ret.update_nodes();
        ret
    }
}
impl NetGenome {
    pub fn pool(&self) -> usize {
        let c = self.color;
        let max = c[0].max(c[1]).max(c[2]);
        if c[0] == max {
            0
        } else if c[1] == max {
            1
        } else {
            2
        }
    }
}
impl Genome for NetGenome {
    fn mix(&self, p2: &NetGenome) -> NetGenome {
        let mut ret = self.clone();
        // let i = self.pool();
        let i = random::<usize>() % 3;
        // ret.color[i] = ret.color[i] * 0.3 + p2.color[i] * 0.7;
        ret.color[i] = ret.color[i] * 0.1 + p2.color[i] * 0.9;
        // ret.color[i] = p2.color[i];
        // return ret;
        for (i, links) in &p2.input_links {
            if random() {
                ret.input_links.insert(i.clone(), links.clone());
            }
        }
        for (i, links) in &p2.hidden_links {
            if random() {
                ret.hidden_links.insert(i.clone(), links.clone());
            }
        }
        ret
    }
}
