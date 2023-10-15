use rand::{distributions::Standard, prelude::Distribution, random, Rng};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Hash, Eq, Serialize, Deserialize)]
pub enum NeighbourType {
    Any,
    Empty,
    Friend,
    Enemy,
    Pool(usize),
}

impl Distribution<NeighbourType> for Standard {
    fn sample<R: Rng + ?Sized>(&self, _rng: &mut R) -> NeighbourType {
        let actions = [
            NeighbourType::Any,
            NeighbourType::Friend,
            NeighbourType::Enemy,
            NeighbourType::Pool(random::<usize>() % 3),
        ];
        let i = random::<usize>() % actions.len();
        *actions.get(i).unwrap()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Hash, Eq, Serialize, Deserialize)]
pub enum Sensor {
    Osc(u8),
    Loc {
        x: bool,
    },
    Neighbour {
        vert: bool,
        incr: bool,
        kind: NeighbourType,
    },
    Bias(i8),
    Random,
    Null,
    Alive,
    Dead,
    // Life,
}

impl Distribution<Sensor> for Standard {
    fn sample<R: Rng + ?Sized>(&self, _rng: &mut R) -> Sensor {
        let actions = [
            // Sensor::Loc { x: false },
            Sensor::Loc { x: random() },
            // Sensor::Osc(random()),
            // Sensor::Bias(random()),
            // Sensor::Random,
            // Sensor::Null,
            Sensor::Neighbour {
                vert: random(),
                incr: random(),
                kind: random(),
            },
            // Sensor::Alive,
            // Sensor::Dead,
        ];
        let i = random::<usize>() % actions.len();
        *actions.get(i).unwrap()
    }
}
