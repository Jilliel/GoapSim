use crate::world::World;
use std::time::{Duration, Instant};

pub trait System {
    fn initialize(&mut self, world: &World);
    fn run(&mut self, world: &mut World);
}

pub struct SystemEntry {
    pub system: Box<dyn System>,
    pub usage: Duration,
}

#[derive(Default)]
pub struct SystemRunner {
    pub systems: Vec<SystemEntry>,
}

impl SystemRunner {
    pub fn add(&mut self, system: Box<dyn System>) {
        self.systems.push(SystemEntry {
            system,
            usage: Duration::ZERO,
        });
    }

    pub fn initialize(&mut self, world: &World) {
        for entry in &mut self.systems {
            entry.system.initialize(world);
        }
    }

    pub fn execute(&mut self, world: &mut World) {
        for entry in &mut self.systems {
            let start = Instant::now();
            entry.system.run(world);
            entry.usage += start.elapsed();
        }
    }
}

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

    pub fn add(&mut self, system: impl System + 'static) {
        self.runner.add(Box::new(system));
    }

    pub fn tick(&mut self) {
        self.runner.execute(&mut self.world);
    }
}
