use crate::{
    terrain::ResourcePile,
    village::{Village, Villager},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct World {
    pub villages: Vec<Village>,
    pub villagers: Vec<Villager>,
    pub resources: Vec<ResourcePile>,
}
