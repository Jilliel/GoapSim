use crate::{item::Inventory, math::Vec2};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Health {
    pub cur: u32,
    pub max: u32,
}

impl Health {
    pub fn new(max: u32) -> Self {
        Self { cur: max, max }
    }

    pub fn damage(&mut self, amount: u32) {
        self.cur = self.cur.saturating_sub(amount);
    }

    pub fn heal(&mut self, amount: u32) {
        self.cur = self.cur.saturating_add(amount).min(self.max);
    }

    pub fn dead(&self) -> bool {
        self.cur == 0
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Villager {
    pos: Vec2,
    health: Health,
}

impl Villager {
    pub fn new(pos: Vec2, health: u32) -> Self {
        Self {
            pos,
            health: Health::new(health),
        }
    }

    pub fn position(&self) -> Vec2 {
        self.pos
    }

    pub fn damage(&mut self, amount: u32) {
        self.health.damage(amount);
    }

    pub fn heal(&mut self, amount: u32) {
        self.health.heal(amount);
    }

    pub fn dead(&self) -> bool {
        self.health.dead()
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Village {
    inv: Inventory,
}

impl Village {
    pub fn new() -> Self {
        Self {
            inv: Inventory::default(),
        }
    }
}
