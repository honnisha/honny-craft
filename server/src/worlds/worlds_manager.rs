use bevy::prelude::Resource;
use bevy_ecs::prelude::Entity;
use dashmap::DashMap;

use crate::{
    entities::entity::{Indentifier, NetworkComponent, Position},
    network::player_container::PlayerMut,
};

use super::world_manager::WorldManager;

#[derive(Resource)]
pub struct WorldsManager {
    worlds: DashMap<String, WorldManager>,
}

impl Default for WorldsManager {
    fn default() -> Self {
        WorldsManager { worlds: DashMap::new() }
    }
}

impl WorldsManager {
    pub fn has_world_with_slug(&self, slug: &String) -> bool {
        self.worlds.contains_key(slug)
    }

    pub fn create_world(&mut self, slug: String) -> Result<(), String> {
        if self.worlds.contains_key(&slug) {
            return Err(format!("World with slug \"{}\" already exists", slug));
        }
        self.worlds.insert(slug.clone(), WorldManager::new(slug));
        Ok(())
    }

    pub fn count(&self) -> usize {
        self.worlds.len()
    }

    pub fn get_worlds(&self) -> &DashMap<String, WorldManager> {
        &self.worlds
    }

    pub fn _get_world_manager(&self, key: &String) -> dashmap::mapref::one::Ref<'_, String, WorldManager> {
        self.worlds.get(key).unwrap()
    }

    pub fn get_world_manager_mut(&self, key: &String) -> dashmap::mapref::one::RefMut<'_, String, WorldManager> {
        self.worlds.get_mut(key).unwrap()
    }

    pub fn spawn(&mut self, player_network: &mut PlayerMut, world_slug: &String, x: f32, y: f32, z: f32) {
        let mut world_manager = self.get_world_manager_mut(world_slug);
        world_manager.get_world_mut().spawn((
            Indentifier::default(),
            Position::new(x, y, z),
            NetworkComponent::new(player_network.get_client_id().clone()),
        ));
        player_network.current_world = Some(world_slug.clone());
    }

    pub fn despawn(&mut self, player_network: &mut PlayerMut) {
        let current_world = match player_network.current_world.as_ref() {
            Some(c) => c,
            None => {
                return;
            },
        };
        let mut world_manager = self.get_world_manager_mut(&current_world);
        let mut world = world_manager.get_world_mut();
        let mut query = world.query::<(Entity, &NetworkComponent)>();

        let mut obj_for_destroy = Vec::new();
        for (entity, network_component) in query.iter_mut(&mut world) {
            if network_component.client_id == *player_network.get_client_id() {
                obj_for_destroy.push(entity.clone());
                continue;
            }
        }

        for entity in obj_for_destroy.drain(..) {
            world.despawn(entity);
        }

        player_network.current_world = None;
    }
}
