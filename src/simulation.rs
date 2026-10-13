use crate::{physics::PhysicModule, world::World};

#[derive(Debug, Clone)]
pub struct Simulation {
    pub world: World,
    pub physic: PhysicModule,
}

impl Simulation {
    pub fn new(world: World) -> Self {
        Self {
            world,
            physic: PhysicModule::default(),
        }
    }

    pub fn tick(&mut self) {
        self.physic.init(&self.world);
        
    }
}
