use crate::{
    systems::{System, SystemRunner},
    world::World,
};

pub struct Simulation {
    pub world: World,
    pub runner: SystemRunner,
}

impl Simulation {
    pub fn new(world: World) -> Self {
        Self {
            world,
            runner: SystemRunner::default(),
        }
    }

    pub fn add<S>(&mut self, system: S)
    where
        S: System + 'static,
    {
        self.runner.add(Box::new(system));
    }

    pub fn tick(&mut self) {
        self.runner.execute(&mut self.world);
    }

    pub fn run(&mut self, iter: u32) {
        for _ in 0..iter {
            self.tick();
        }
    }
}
