use super::System;
use crate::world::World;

#[derive(Debug, Default, Clone, Copy)]
pub struct CleanupSystem;

impl System for CleanupSystem {
    fn initialize(&mut self, _world: &World) {}

    fn run(&mut self, world: &mut World) {
        world.villagers.retain(|villager| !villager.dead());
        world.resources.retain(|resource| !resource.empty());
    }
}
