use rand::random;
use serde::{Deserialize, Serialize};

use crate::{
    genome::HasGenome,
    net::{Net, NetGenome},
    simulation::CellMapper,
    world::World,
};

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Replicant {
    pub pos: (i32, i32),
    pub net: Net,
    pub time: usize,
    pub moves: usize,
}

impl Replicant {
    pub fn dist(&self, rep: &Self) -> i32 {
        (self.pos.0 - rep.pos.0).abs() + (self.pos.1 - rep.pos.1).abs()
    }
    pub fn is_alive(&self, world: &World, map: &CellMapper) -> bool {
        // return true;
        let x = self.pos.0;
        let y = self.pos.1;
        let px = self.pos.0 as f32;
        let py = self.pos.1 as f32;
        let w = world.width as f32;
        let h = world.height as f32;
        // if x > world.width / 2 {
        //     return true;
        // }
        // return false;
        // return self.pos.0 > world.width/2;
        // || self.pos.0 < world.width/4;

        // if x >= world.width-1 || x <=0 {
        //     return false;
        // }
        // if y >= world.height-1 || y <=0 {
        //     return false;
        // }
        // if px > w * 0.95 {
        //     return false;
        // }
        if px < w * 0.5 {
            return false;
        }
        return true;
        // if py > h * 0.95 {
        //     return false;
        // }
        // if py < h * 0.05 {
        //     return false;
        // }

        let friend = self.net.pool();
        let ally = (friend + 1) % 3;
        let enemy = (friend + 2) % 3;
        let red = 0;
        let green = 1;
        let blue = 2;
        // return map.isexcept(x-1, y, friend) as i8
        //     + map.isexcept(x+1, y, friend) as i8
        //     + map.isexcept(x, y+1, friend) as i8
        //     + map.isexcept(x, y-1, friend) as i8
        //     > 2;
        // if pool == 1 {
        //     return map.isexcept(x-1, y, pool) as i8
        //         + map.isexcept(x+1, y, pool) as i8
        //         + map.isexcept(x, y+1, pool) as i8
        //         + map.isexcept(x, y-1, pool) as i8
        //         > 2;
        // }
        // let lrtb_empty_count = !map.has(x - 1, y) as i8
        //     + !map.has(x + 1, y) as i8
        //     + !map.has(x, y + 1) as i8
        //     + !map.has(x, y - 1) as i8;
        let lrtb_friend_count = map.is(x - 1, y, friend) as i8
            + map.is(x + 1, y, friend) as i8
            + map.is(x, y + 1, friend) as i8
            + map.is(x, y - 1, friend) as i8;
        // let lrtb_ally_count = map.is(x - 1, y, (pool + 1) % 3) as i8
        //     + map.is(x + 1, y, (pool + 1) % 3) as i8
        //     + map.is(x, y + 1, (pool + 1) % 3) as i8
        //     + map.is(x, y - 1, (pool + 1) % 3) as i8;
        let enemy_count = map.is(x - 1, y, enemy) as i8
            + map.is(x + 1, y, enemy) as i8
            + map.is(x, y + 1, enemy) as i8
            + map.is(x - 1, y + 1, enemy) as i8
            + map.is(x + 1, y + 1, enemy) as i8
            + map.is(x, y - 1, enemy) as i8
            + map.is(x - 1, y - 1, enemy) as i8
            + map.is(x + 1, y - 1, enemy) as i8;
        let friend_count = map.is(x - 1, y, friend) as i8
            + map.is(x + 1, y, friend) as i8
            + map.is(x, y + 1, friend) as i8
            + map.is(x - 1, y + 1, friend) as i8
            + map.is(x + 1, y + 1, friend) as i8
            + map.is(x, y - 1, friend) as i8
            + map.is(x - 1, y - 1, friend) as i8
            + map.is(x + 1, y - 1, friend) as i8;

        let ally_count = map.is(x - 1, y, ally) as i8
            + map.is(x + 1, y, ally) as i8
            + map.is(x, y + 1, ally) as i8
            + map.is(x - 1, y + 1, ally) as i8
            + map.is(x + 1, y + 1, ally) as i8
            + map.is(x, y - 1, ally) as i8
            + map.is(x - 1, y - 1, ally) as i8
            + map.is(x + 1, y - 1, ally) as i8;
        let red_count = map.is(x - 1, y, red) as i8
            + map.is(x + 1, y, red) as i8
            + map.is(x, y + 1, red) as i8
            + map.is(x - 1, y + 1, red) as i8
            + map.is(x + 1, y + 1, red) as i8
            + map.is(x, y - 1, red) as i8
            + map.is(x - 1, y - 1, red) as i8
            + map.is(x + 1, y - 1, red) as i8;
        let green_count = map.is(x - 1, y, green) as i8
            + map.is(x + 1, y, green) as i8
            + map.is(x, y + 1, green) as i8
            + map.is(x - 1, y + 1, green) as i8
            + map.is(x + 1, y + 1, green) as i8
            + map.is(x, y - 1, green) as i8
            + map.is(x - 1, y - 1, green) as i8
            + map.is(x + 1, y - 1, green) as i8;
        let blue_count = map.is(x - 1, y, blue) as i8
            + map.is(x + 1, y, blue) as i8
            + map.is(x, y + 1, blue) as i8
            + map.is(x - 1, y + 1, blue) as i8
            + map.is(x + 1, y + 1, blue) as i8
            + map.is(x, y - 1, blue) as i8
            + map.is(x - 1, y - 1, blue) as i8
            + map.is(x + 1, y - 1, blue) as i8;
        let any_count = map.has(x - 1, y) as i8
            + map.has(x + 1, y) as i8
            + map.has(x, y + 1) as i8
            + map.has(x - 1, y + 1) as i8
            + map.has(x + 1, y + 1) as i8
            + map.has(x, y - 1) as i8
            + map.has(x - 1, y - 1) as i8
            + map.has(x + 1, y - 1) as i8;
        let empty_count = !map.has(x - 1, y) as i8
            + !map.has(x + 1, y) as i8
            + !map.has(x, y + 1) as i8
            + !map.has(x - 1, y + 1) as i8
            + !map.has(x + 1, y + 1) as i8
            + !map.has(x, y - 1) as i8
            + !map.has(x - 1, y - 1) as i8
            + !map.has(x + 1, y - 1) as i8;

        // return any_count > 0;
        // return friend_count > 3;
        // if friend == 0 {
        //     return enemy_count > 0 || ally_count > 0;
        // }
        return friend_count > 2 && friend_count < 5 && enemy_count == 0 && ally_count == 0;
        // if friend == 1 {
        //     return friend_count > 1 && enemy_count == 0;
        // }


        // if friend == green {
        //     return red_count == 0 && blue_count == 1;
        // }
        // if friend == blue {
        //     return red_count == 0 && green_count > 1;
        // }
        // return lrtb_friend_count == 4 && enemy_count == 0;

        // return ((lrtb_friend_count == 1 && friend_count == 1)
        //     || (lrtb_friend_count == 2 && friend_count <= 2));
        // return false
        // || (lrtb_friend_count == 3 && friend_count <= 3 + 1);
        // || (lrtb_friend_count == 4 && friend_count <= 4 + 1));

        // return friend_count > 2 && (enemy_count+ally_count) < 2;
        // return ;
        // return map.is(x, y - 1, friend);

        // let has_friend_x = map.is(x - 1, y, pool) || map.is(x + 1, y, pool);
        // let has_friend_y = map.is(x, y + 1, pool) || map.is(x, y - 1, pool);
        // let has_friend = has_friend_x || has_friend_y;
        // let has_enemy_x = (map.has(x - 1, y) && !map.is(x - 1, y, pool))
        // || (map.has(x + 1, y) && !map.is(x + 1, y, pool));
        // let has_enemy_y = (map.has(x, y + 1) && !map.is(x, y + 1, pool))
        // || (map.has(x, y - 1) && !map.is(x, y - 1, pool));
        // let has_enemy = has_enemy_x || has_enemy_y;
        // // return has_enemy_x && has_friend_y;
        // return has_friend && !has_enemy;

        // return map.is(x - 1, y, pool);
        // if self.net.pool() > 0 {
        //     return
        //         !map.has(x-1, y)
        //     && !map.has(x+1, y)
        //     && !map.has(x, y+1)
        //     && !map.has(x, y-1)
        // } else {
        //     return
        //         map.has(x-1, y)
        //     || map.has(x+1, y)
        //     || map.has(x, y+1)
        //     || map.has(x, y-1)
        // }
        // ;
        // return
        //  !map.has(x+1, y)
        //  && !map.has(x-1, y)
        //  && map.has(x, y+1)
        //  && !map.has(x, y-1)
        // ;
        // return true;
        // return
        //  map.has(x, y-1)
        //  && !map.has(x, y+1);
        // return (map.has(x + 1, y) == map.has(x - 1, y) && map.has(x, y + 1) != map.has(x, y - 1))
        //     || (map.has(x + 1, y) != map.has(x - 1, y) && map.has(x, y + 1) == map.has(x, y - 1));
        // let p = self.pos.0 as f32;
        // (-3.14*0.2 + p*0.6).sin() > 0.05
        // px > (world.width as f32)*0.2
        // && px < (world.width as f32)*0.8
        // && py > (world.height as f32)*0.2
        // && py < (world.height as f32)*0.8
        // py < world.height as f32 * 0.1
        //     || (px > world.width as f32 * 0.8 && py > world.height as f32 * 0.8)
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
