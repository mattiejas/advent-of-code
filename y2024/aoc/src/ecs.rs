use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

pub trait Component: Clone + Any {}
type Entity = u32;

pub struct World {
    entities: Vec<Entity>,
    components: HashMap<TypeId, Box<dyn Any>>,
    entity_components: HashMap<Entity, Vec<TypeId>>,
}

impl World {
    pub fn new() -> Self {
        Self {
            entities: Vec::new(),
            components: HashMap::new(),
            entity_components: HashMap::new(),
        }
    }

    pub fn create_entity(&mut self) -> Entity {
        let entity = self.entities.len() as Entity;
        self.entities.push(entity);
        entity
    }

    pub fn add_component<T: Component>(&mut self, entity: Entity, component: T) {
        let type_id = TypeId::of::<T>();
        self.components.insert(type_id, Box::new(component));
        self.entity_components
            .entry(entity)
            .or_insert_with(Vec::new)
            .push(type_id);
    }

    pub fn get_component<T: Component>(&self, entity: Entity) -> Option<&T> {
        let type_id = TypeId::of::<T>();
        if let Some(component) = self.components.get(&type_id) {
            component.downcast_ref::<T>()
        } else {
            None
        }
    }

    pub fn remove_component<T: Component>(&mut self, entity: Entity) {
        let type_id = TypeId::of::<T>();
        self.components.remove(&type_id);
        if let Some(components) = self.entity_components.get_mut(&entity) {
            components.retain(|&id| id != type_id);
        }
    }
}

type System = fn(&mut World);

pub struct SystemManager {
    systems: Vec<System>,
}

impl SystemManager {
    pub fn new() -> Self {
        Self {
            systems: Vec::new(),
        }
    }

    pub fn add_system(&mut self, system: System) {
        self.systems.push(system);
    }

    pub fn run_systems(&mut self, world: &mut World) {
        for system in self.systems.iter() {
            system(world);
        }
    }
}
