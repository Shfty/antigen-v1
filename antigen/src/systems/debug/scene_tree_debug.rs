use std::cell::{Ref, RefMut};

use store::{NoField, StoreQuery};

use crate::{
    components::Name,
    entity_component_system::{SystemError, SystemTrait},
};
use crate::{
    components::{ChildEntitiesData, DebugExclude, DebugSceneTree, ParentEntity},
    entity_component_system::{ComponentStore, EntityID},
};

#[derive(Debug)]
pub struct SceneTreeDebug;

impl SystemTrait for SceneTreeDebug {
    fn run(&mut self, db: &mut ComponentStore) -> Result<(), SystemError> {
        let mut debug_entities: Vec<EntityID> =
            StoreQuery::<(EntityID, NoField<DebugExclude>)>::iter(db.as_ref())
                .map(|(entity_id, _)| entity_id)
                .collect();
        debug_entities.sort();

        let root_entities: Vec<EntityID> = debug_entities
            .iter()
            .filter(|entity_id| !db.contains_type_key::<ParentEntity>(entity_id))
            .copied()
            .collect();

        let mut scene_tree_strings: Vec<String> = Vec::new();

        fn traverse_tree(
            db: &mut ComponentStore,
            entity_id: &EntityID,
            scene_tree_strings: &mut Vec<String>,
            mut padding: Vec<String>,
        ) -> Result<(), String> {
            let depth = padding.len();

            let prefix: String = if depth == 0 {
                "".to_string()
            } else {
                padding.iter().cloned().collect::<String>() + " "
            };

            for string in padding.iter_mut() {
                match string.as_str() {
                    "└" => *string = "  ".into(),
                    "├" => *string = "│ ".into(),
                    _ => (),
                };
            }

            let label: String = match db.get::<Name>(entity_id) {
                Some(name) => (**name).clone(),
                None => "Entity".into(),
            };

            let label = format!("{}:\t{}{}", entity_id, &prefix, label);
            scene_tree_strings.push(label);

            let child_ids: Vec<EntityID>;
            if let Some(child_entities) = db.get::<ChildEntitiesData>(entity_id) {
                child_ids = child_entities
                    .iter()
                    .filter(|child_id| !db.contains_type_key::<DebugExclude>(child_id))
                    .copied()
                    .collect();
            } else {
                child_ids = Vec::new();
            }

            for (i, child_entity) in child_ids.iter().enumerate() {
                let mut padding = padding.clone();
                padding.push(
                    if child_ids.len() == 1 || i == child_ids.len() - 1 {
                        "└"
                    } else {
                        "├"
                    }
                    .into(),
                );
                traverse_tree(db, child_entity, scene_tree_strings, padding)?;
            }

            Ok(())
        }

        // Populate strings for debug scene tree entities
        for root_entity in &root_entities {
            traverse_tree(db, root_entity, &mut scene_tree_strings, Vec::new())?;
        }

        StoreQuery::<(EntityID, Ref<DebugSceneTree>, RefMut<Vec<String>>)>::iter(db.as_ref())
            .for_each(|(_, _, mut strings)| *strings = scene_tree_strings.clone());

        Ok(())
    }
}
