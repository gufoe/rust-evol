use serde::{Deserialize, Serialize};

use crate::{
    net::{NetGenome, Net}, genome::{HasGenome}, world::World,
};

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Replicant {
    pub pos: (i32, i32),
    pub net: Net,
    pub time: usize,
}

impl Replicant {
    pub fn is_alive(&self, world: &World) -> bool {
        let px = self.pos.0 as f32;
        let py = self.pos.1 as f32;
        // let p = self.pos.0 as f32;
        // (-3.14*0.2 + p*0.6).sin() > 0.05
        // px > (world.width as f32)*0.2
        // && px < (world.width as f32)*0.8
        // && py > (world.height as f32)*0.2
        // && py < (world.height as f32)*0.8
                py < world.height as f32 * 0.5
                // || px < world.width/10
                // ((px / 30) % 2 == 0)
                // && ((p.1 / 30) % 2 == 0)
                // && px.min(p.1) > 20
                // && px.max(p.1) < world.width-20
                // px > world.width / 4
                //     && px < world.width / 4 * 3
                //     && p.1 > world.height / 4
                //     && p.1 < world.height / 4 * 3
    }
}
impl HasGenome<NetGenome> for Replicant {
    fn from_genome(genome: &NetGenome) -> Self {
        Self {
            net: Net::from_genome(genome),
            ..Default::default()
        }
    }
    fn to_genome(&self) -> NetGenome {
        self.net.to_genome()
    }
}
