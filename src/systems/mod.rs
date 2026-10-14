use crate::world::World;

pub mod cleanup;
pub mod physics;

pub use cleanup::CleanupSystem;
pub use physics::PhysicSystem;

pub trait System {
    fn initialize(&mut self, world: &World);
    fn run(&mut self, world: &mut World);
}

#[derive(Default)]
pub struct SystemRunner {
    pub systems: Vec<Box<dyn System>>,
}

impl SystemRunner {
    pub fn add(&mut self, system: Box<dyn System>) {
        self.systems.push(system);
    }

    pub fn initialize(&mut self, world: &World) {
        for system in &mut self.systems {
            system.initialize(world);
        }
    }

    pub fn execute(&mut self, world: &mut World) {
        for system in &mut self.systems {
            system.run(world);
        }
    }
}
