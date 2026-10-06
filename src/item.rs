use rustc_hash::FxHashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Resource {
    Wood,
    Iron,
    Gold,
    Stone,
}

#[derive(Debug, Clone, Copy)]
pub struct ResourceStack {
    resource: Resource,
    amount: u32,
}

impl ResourceStack {
    pub fn new(resource: Resource, amount: u32) -> Self {
        Self { resource, amount }
    }

    pub fn resource(&self) -> Resource {
        self.resource
    }

    pub fn amount(&self) -> u32 {
        self.amount
    }

    pub fn take(&mut self, amount: u32) -> ResourceStack {
        let available = amount.min(self.amount);
        self.amount -= available;
        ResourceStack::new(self.resource, available)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Item {
    Wood,
    Iron,
    Gold,
    Stone,
}

#[derive(Debug, Clone, Copy)]
pub struct ItemStack {
    item: Item,
    amount: u32,
}

impl From<ResourceStack> for ItemStack {
    fn from(stack: ResourceStack) -> ItemStack {
        let item = match stack.resource {
            Resource::Wood => Item::Wood,
            Resource::Iron => Item::Iron,
            Resource::Gold => Item::Gold,
            Resource::Stone => Item::Stone,
        };
        ItemStack::new(item, stack.amount)
    }
}

impl ItemStack {
    pub fn new(item: Item, amount: u32) -> Self {
        Self { item, amount }
    }

    pub fn item(&self) -> Item {
        self.item
    }

    pub fn amount(&self) -> u32 {
        self.amount
    }
}

#[derive(Debug, Clone, Default)]
pub struct Inventory {
    inner: FxHashMap<Item, u32>,
}

impl Inventory {
    pub fn get(&self, item: Item) -> Option<u32> {
        self.inner.get(&item).copied()
    }

    pub fn add(&mut self, stack: ItemStack) {
        *self.inner.entry(stack.item).or_default() += stack.amount;
    }
}
