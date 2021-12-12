use rand::{distributions::Standard, prelude::Distribution, Rng, random};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Hash, Eq, Serialize, Deserialize)]
pub enum Sensor {
    Osc,
    LocX,
    LocY,
    Bias(i8),
    Random,
    // Life,
}

impl Distribution<Sensor> for Standard {
    fn sample<R: Rng + ?Sized>(&self, _rng: &mut R) -> Sensor {
        let actions = [
            // Sensor::Osc,
            Sensor::LocX,
            Sensor::LocY,
            // Sensor::Bias(random()),
            Sensor::Random,
            // Sensor::Life,
        ];
        let i = random::<usize>() % actions.len();
        *actions.get( i).unwrap()
    }
}