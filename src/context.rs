use crate::components::*;
use slotmap::{DefaultKey, SecondaryMap, SlotMap};

pub struct Context {
    pub entity_type_components: SlotMap<DefaultKey, EntityType>,
    pub ai_components: SecondaryMap<DefaultKey, AI>,
    pub damageable_components: SecondaryMap<DefaultKey, Damageable>,
    pub displayable_components: SecondaryMap<DefaultKey, Displayable>,
    pub growth_components: SecondaryMap<DefaultKey, Growth>,
    pub loot_components: SecondaryMap<DefaultKey, Loot>,
    pub physical_properties_components: SecondaryMap<DefaultKey, PhysicalProperties>,
    pub position_components: SecondaryMap<DefaultKey, Position>,
    pub storage_child_components: SecondaryMap<DefaultKey, StorageChild>,
    pub storage_components: SecondaryMap<DefaultKey, Storage>,
    pub weapon_components: SecondaryMap<DefaultKey, Weapon>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            entity_type_components: SlotMap::new(),
            ai_components: SecondaryMap::new(),
            damageable_components: SecondaryMap::new(),
            displayable_components: SecondaryMap::new(),
            growth_components: SecondaryMap::new(),
            loot_components: SecondaryMap::new(),
            physical_properties_components: SecondaryMap::new(),
            position_components: SecondaryMap::new(),
            storage_child_components: SecondaryMap::new(),
            storage_components: SecondaryMap::new(),
            weapon_components: SecondaryMap::new(),
        }
    }

    pub fn delete_entity(&mut self, entity: DefaultKey) {
        // Account for Loot and StorageChild
        unimplemented!("TODO")
    }
}
