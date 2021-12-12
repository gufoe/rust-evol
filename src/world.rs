use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct World {
    pub width: i32,
    pub height: i32,
}
