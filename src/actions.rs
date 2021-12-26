use rand::{prelude::Distribution, Rng, distributions::Standard, random};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Hash, Eq, Serialize, Deserialize)]
pub enum Action {
    IncX,
    DecX,
    IncY,
    DecY,
}

impl Distribution<Action> for Standard {
    fn sample<R: Rng + ?Sized>(&self, _rng: &mut R) -> Action {
        let actions = [
            Action::IncX,
            Action::DecX,
            Action::IncY,
            Action::DecY,
        ];
        let i = random::<usize>() % actions.len();
        *actions.get( i).unwrap()
    }
}