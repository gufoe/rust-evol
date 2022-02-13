use std::{
    collections::{hash_map::DefaultHasher, HashMap, HashSet},
    hash::{Hash, Hasher},
};

use rand::random;
use serde::{Deserialize, Serialize};

use crate::{
    actions::Action,
    genome::{Genome, HasGenome},
    input::Sensor,
    pool::{Allele, AlleleID},
    rng::rand_f32,
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
    pub fn add(&mut self, num: f32) {
        self.charge += num;
    }
    fn fire(&mut self) -> bool {
        let out = self.output();
        if self.output().abs() < 0.9 {
            false
        } else {
            self.charge = 0.0;
            true
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NeuralLink {
    pub inverse: bool,
    pub weight: f32,
}

impl Hash for NeuralLink {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.inverse.hash(state);
        ((self.weight * 100.0).round() as i32).hash(state);
    }
}

impl NeuralLink {
    fn new() -> Self {
        Self {
            weight: rand_f32() * 4.0,
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
            NeuralTarget::Hidden(action) => {
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
pub struct NeuralNode {
    pub inputs: HashMap<NeuralSource, NeuralLink>,
}
impl Hash for NeuralNode {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        for (target, link) in &self.inputs {
            target.hash(state);
            link.hash(state);
        }
    }
}

impl Allele<NeuralTarget> for NeuralNode {
    fn get_allele_id(&self) -> AlleleID {
        let mut s = DefaultHasher::new();
        self.hash(&mut s);
        s.finish()
    }
    fn get_gene_requirements(&self) -> Vec<NeuralTarget> {
        self.inputs
            .iter()
            .filter_map(|(target, _link)| match target {
                NeuralSource::Hidden(x) => Some(NeuralTarget::Hidden(x.clone())),
                _ => None,
            })
            .collect()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Net {
    // pub input_neurons: HashMap<Sensor, Neuron>,
    pub nodes: HashMap<NeuralTarget, NeuralNode>,
    pub sensors: HashMap<Sensor, f32>,
    pub state: HashMap<NeuralTarget, Neuron>,
    pub color: [f32; 3],
}

impl Default for Net {
    fn default() -> Self {
        let mut color = [0.0; 3];
        *color.get_mut(random::<usize>() % 3).unwrap() = random::<f32>().abs();
        Self {
            color,
            nodes: Default::default(),
            state: Default::default(),
            sensors: Default::default(),
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
    // pub fn update_nodes(&mut self) {
    //     self.nodes = NetNodes::default();
    //     for links in self.input_links.values() {
    //         for (target, _link) in &links.outputs {
    //             self.nodes.create_link_node(&target);
    //         }
    //     }
    //     for links in self.hidden_links.values() {
    //         for (target, _link) in &links.outputs {
    //             self.nodes.create_link_node(&target);
    //         }
    //     }
    // }
    // pub fn clear(&mut self) {
    //     let clone: Vec<_> = self.hidden_links.keys().cloned().collect();
    //     for nid in &clone {
    //         if !self.nodes.hidden.contains_key(nid) {
    //             self.hidden_links.remove(nid);
    //         }
    //     }
    // }
    // pub fn randomize(&mut self) {
    //     let i = random::<usize>() % self.color.len();
    //     let c = self.color.get_mut(i).unwrap();
    //     *c += (random::<f32>() * 2.0 - 1.0) * 0.2;
    //     if *c > 1.0 {
    //         *c = 1.0 - (*c - 1.0);
    //     }
    //     if *c < 0.0 {
    //         *c = c.abs();
    //     }
    //     *c = c.min(1.0).max(0.0);
    //     let max = self.color[0].max(self.color[1]).max(self.color[2]);
    //     for x in &mut self.color {
    //         if *x != max {
    //             *x -= 0.01;
    //             *x = x.min(1.0).max(0.0);
    //         }
    //     }
    //     // self.add_link_from_sensor(random(), NeuralTarget::Action(random()));
    //     // return;

    //     if random() {
    //         for _ in 0..1 {
    //             self.add_link_from_sensor(random(), NeuralTarget::Action(random()));
    //         }
    //     }
    //     if !self.nodes.hidden.is_empty() && random() {
    //         let hid: Vec<&String> = self.nodes.hidden.keys().collect();
    //         let hid: String = hid[random::<usize>() % hid.len()].clone();
    //         self.add_link_from_sensor(random(), NeuralTarget::Hidden(hid));
    //     }
    //     if !self.nodes.hidden.is_empty() && random() {
    //         let hid: Vec<&String> = self.nodes.hidden.keys().collect();
    //         let hid: String = hid[random::<usize>() % hid.len()].clone();
    //         self.add_hidden_link(hid, random());
    //     }
    //     if random() {
    //         for _ in 0..1 {
    //             let h = rand_h();
    //             self.add_link_from_sensor(random(), NeuralTarget::Hidden(h.clone()));
    //             self.add_hidden_link(h.clone(), random());
    //         }
    //     }
    //     for _ in 0..(random::<usize>() % 2) {
    //         if self.hidden_links.is_empty() {
    //             continue;
    //         }
    //         let keys: Vec<_> = self.hidden_links.keys().collect();
    //         let i = random::<usize>() % keys.len();
    //         let key = keys[i].clone();
    //         self.hidden_links.remove(&key).unwrap();
    //     }
    //     for _ in 0..(random::<usize>() % 2) {
    //         if self.input_links.is_empty() {
    //             continue;
    //         }
    //         let keys: Vec<_> = self.input_links.keys().collect();
    //         let i = random::<usize>() % keys.len();
    //         let key = keys[i].clone();
    //         self.input_links.remove(&key).unwrap();
    //     }
    //     self.update_nodes();
    //     self.clear();
    //     // print!("\r{:?}", self.nodes.hidden.len());
    // }
    // pub fn add_link_from_sensor(&mut self, source: Sensor, target: NeuralTarget) {
    //     let link = NeuralLink::new();
    //     match self.input_links.get_mut(&source) {
    //         Some(links) => {
    //             links.outputs.insert(target, link);
    //         }
    //         None => {
    //             let mut links = HashMap::new();
    //             links.insert(target, link);
    //             self.input_links
    //                 .insert(source, NeuralNode { outputs: links });
    //         }
    //     };
    // }
    // pub fn add_hidden_link(&mut self, source: String, action: Action) {
    //     let link = NeuralLink::new();
    //     let target = NeuralTarget::Action(action);
    //     match self.hidden_links.get_mut(&source) {
    //         Some(links) => {
    //             links.outputs.insert(target, link);
    //         }
    //         None => {
    //             let mut links = HashMap::new();
    //             links.insert(target, link);
    //             self.hidden_links
    //                 .insert(source, NeuralNode { outputs: links });
    //         }
    //     };
    // }
    pub fn tick(&mut self) -> Vec<Action> {
        let state = self.state.clone();
        self.discharge();
        // input.iter().for_each(|(sensor, x)| {
        //     let links = clone.get(sensor).unwrap();
        //     self.apply_synapse(*x, &links.outputs);
        // });
        let mut actions = vec![];
        self.nodes.iter().for_each(|(target, node)| {
            let sum = node
                .inputs
                .iter()
                .map(|(source, link)| {
                    let mut x = match source {
                        NeuralSource::Hidden(hid) => {
                            state[&NeuralTarget::Hidden(hid.clone())].output()
                        }
                        NeuralSource::Sensor(sensor) => *self.sensors.get(sensor).unwrap(),
                    };
                    if link.inverse {
                        x = 1.0 - x;
                    }
                    (x * link.weight).tanh()
                })
                .sum::<f32>() / node.inputs.len() as f32;
            let state = self.state.get_mut(target).unwrap();
            state.add(sum);
            if let NeuralTarget::Action(action) = target {
                if state.fire() {
                    actions.push(action.clone());
                }
            }
        });

        actions
        // return self
        //     .nodes
        //     .output
        //     .iter_mut()
        //     .filter_map(|(k, n)| if n.fires() { Some(k) } else { None })
        //     .cloned()
        //     .collect();
    }
    // fn apply_synapse(&mut self, x: f32, links: &HashMap<NeuralTarget, NeuralLink>) {
    //     links.iter().for_each(|(target, link)| {
    //         let y = link.output(x);
    //         let neuron = match &target {
    //             NeuralTarget::Action(action) => self.nodes.output.get_mut(action),
    //             NeuralTarget::Hidden(neuron_id) => self.nodes.hidden.get_mut(neuron_id),
    //         };
    //         if let Some(neuron) = neuron {
    //             neuron.charge += y;
    //         }
    //     });
    // }
    fn discharge(&mut self) {
        self.state.values_mut().for_each(|neuron| {
            neuron.discharge();
        });
    }
}

#[derive(Default, Clone, Serialize)]
pub struct NetGenome {
    pub color: [f32; 3],
    pub nodes: HashMap<NeuralTarget, NeuralNode>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub enum NeuralSource {
    Sensor(Sensor),
    Hidden(NeuronID),
}

#[derive(Clone, Debug, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub enum NeuralTarget {
    Hidden(NeuronID),
    Action(Action),
}

impl NetGenome {
    pub fn value(&self) -> serde_json::Value {
        let mut value = serde_json::json!({});
        value["color"] = serde_json::to_value(self.color).unwrap();
        let links: Vec<_> = self
            .nodes
            .iter()
            .map(|(source, links)| {
                (
                    source,
                    links
                        .inputs
                        .iter()
                        .map(|(i, v)| (i.clone(), v.clone()))
                        .collect::<Vec<_>>(),
                )
            })
            .collect();
        value["links"] = serde_json::to_value(links).unwrap();
        value
    }
}
impl HasGenome<NetGenome> for Net {
    fn to_genome(&self) -> NetGenome {
        NetGenome {
            color: self.color.clone(),
            nodes: self.nodes.clone(),
        }
    }

    fn from_genome(genome: &NetGenome) -> Self {
        Net {
            color: genome.color.clone(),
            nodes: genome.nodes.clone(),
            sensors: genome
                .nodes
                .values()
                .flat_map(|node| {
                    node.inputs.keys().filter_map(|x| match x {
                        NeuralSource::Hidden(_) => None,
                        NeuralSource::Sensor(x) => Some((x.clone(), 0.0)),
                    })
                })
                .collect(),
            state: genome
                .nodes
                .keys()
                .map(|target| (target.clone(), Neuron::default()))
                .collect(),
        }
    }
}
impl NetGenome {
    pub fn randomize_color(&mut self) {
        let i = random::<usize>() % self.color.len();
        let c = self.color.get_mut(i).unwrap();
        *c += (random::<f32>() * 2.0 - 1.0) * 0.2;
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
    }
    pub fn randomize(&mut self) {
        if self.nodes.is_empty() || random::<f32>() > 0.9 {
            let mut node = NeuralNode {
                inputs: HashMap::new(),
            };
            node.inputs
                .insert(NeuralSource::Sensor(random()), NeuralLink::new());
            if random() {
                let target = NeuralTarget::Action(random());
                self.nodes.insert(target, node);
            } else {
                // Add hid <- random sensor
                let hid = rand_h();
                let target = NeuralTarget::Hidden(hid.clone());
                self.nodes.insert(target, node);

                // Add random action <- hid
                let mut node2 = NeuralNode {
                    inputs: HashMap::new(),
                };
                node2
                    .inputs
                    .insert(NeuralSource::Hidden(hid), NeuralLink::new());
                self.nodes.insert(NeuralTarget::Action(random()), node2);
            };
        } else {
            let keys: Vec<_> = self.nodes.keys().collect();
            let target = keys[random::<usize>() % keys.len()].clone();
            let node = self.nodes.get_mut(&target).unwrap();
            node.inputs
                .insert(NeuralSource::Sensor(random()), NeuralLink::new());
        }
    }
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
        // // let i = self.pool();
        // let i = random::<usize>() % 3;
        // // ret.color[i] = ret.color[i] * 0.3 + p2.color[i] * 0.7;
        // ret.color[i] = ret.color[i] * 0.1 + p2.color[i] * 0.9;
        // // ret.color[i] = p2.color[i];
        // // return ret;
        // for (i, links) in &p2.links {
        //     if random() {
        //         ret.links.insert(i.clone(), links.clone());
        //     }
        // }
        ret
    }
}
