use crate::item::{ItemStack, ResourceStack};
use crate::math::Vec2;
use crate::item::Resource;

#[derive(Debug, Clone, Copy)]
pub struct ResourcePile {
    pub pos: Vec2,
    pub stack: ResourceStack,
}

impl ResourcePile {
    pub fn new(pos: Vec2, resource: Resource) -> Self {
        Self {
            pos,
            stack: ResourceStack::new(resource, 100),
        }
    }

    pub fn exploit(&mut self, amount: u32) -> ItemStack {
        self.stack.take(amount).into()
    }

    pub fn empty(&self) -> bool {
        self.stack.amount() == 0
    }
}
