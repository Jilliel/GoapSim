use crate::{
    math::{Rect, Vec2},
    simulation::System,
    world::World,
};
use rustc_hash::FxHashMap;

pub const CHUNK_SIZE: f32 = 8.0;

#[derive(Debug, Clone, Default)]
pub struct Chunk {
    pub resources: Vec<usize>,
}

type ChunkCoords = (i32, i32);

#[derive(Debug, Default, Clone)]
pub struct PhysicSystem {
    chunks: FxHashMap<ChunkCoords, Chunk>,
}

impl System for PhysicSystem {
    fn initialize(&mut self, world: &World) {
        self.chunks.clear();
        for (id, resource) in world.resources.iter().enumerate() {
            let rect = Rect::new(resource.position(), Vec2::new(2.0, 2.0));
            for coords in rect.chunks(CHUNK_SIZE) {
                self.chunks.entry(coords).or_default().resources.push(id);
            }
        }
    }

    fn run(&mut self, world: &mut World) {
        for villager in &mut world.villagers {
            let rect = Rect::new(villager.position(), Vec2::new(1.0, 1.0));
            for coords in rect.chunks(CHUNK_SIZE) {
                let Some(chunk) = self.chunks.get(&coords) else {
                    continue;
                };
                for &id in &chunk.resources {
                    let resource = &world.resources[id];
                    let dx = villager.position().x - resource.position().x;
                    let dy = villager.position().y - resource.position().y;
                    let overlap_x = 1.5 - dx.abs();
                    let overlap_y = 1.5 - dy.abs();
                    if overlap_x <= 0.0 || overlap_y <= 0.0 {
                        continue;
                    }

                    let delta = if overlap_x < overlap_y {
                        Vec2::new(overlap_x.copysign(dx), 0.0)
                    } else {
                        Vec2::new(0.0, overlap_y.copysign(dy))
                    };
                    villager.translate(delta);
                }
            }
        }
    }
}
