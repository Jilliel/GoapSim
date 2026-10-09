use crate::item::Resource;
use crate::item::{ItemStack, ResourceStack};
use crate::math::Vec2;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ResourcePile {
    pos: Vec2,
    stack: ResourceStack,
}

impl ResourcePile {
    pub fn new(pos: Vec2, resource: Resource) -> Self {
        Self {
            pos,
            stack: ResourceStack::new(resource, 100),
        }
    }

    pub fn position(&self) -> Vec2 {
        self.pos
    }

    pub fn extract(&mut self, amount: u32) -> ItemStack {
        self.stack.take(amount).into()
    }

    pub fn empty(&self) -> bool {
        self.stack.amount() == 0
    }
}
