use std::cell::{Ref, RefMut};

use store::StoreQuery;

use crate::{
    components::Name,
    entity_component_system::{SystemError, SystemTrait},
};
use crate::{
    components::{ChildEntitiesData, DebugExclude, DebugSceneTree, ParentEntity},
    entity_component_system::{
        system_interface::SystemInterface, EntityComponentDirectory, EntityID,
    },
};

#[derive(Debug)]
pub struct SceneTreeDebug;

impl<CD> SystemTrait<CD> for SceneTreeDebug
where
    CD: EntityComponentDirectory,
{
    fn run(&mut self, db: &mut SystemInterface<CD>) -> Result<(), SystemError>
    where
        CD: EntityComponentDirectory,
    {
        let mut debug_entities: Vec<EntityID> = db
            .entity_component_directory
            .get_entities_by_predicate(|entity_id| {
                !db.entity_has_component::<DebugExclude>(entity_id)
            });
        debug_entities.sort();

        let root_entities: Vec<EntityID> = debug_entities
            .iter()
            .filter(|entity_id| !db.entity_has_component::<ParentEntity>(entity_id))
            .copied()
            .collect();

        let mut scene_tree_strings: Vec<String> = Vec::new();

        fn traverse_tree<CD>(
            db: &mut SystemInterface<CD>,
            entity_id: &EntityID,
            scene_tree_strings: &mut Vec<String>,
            mut padding: Vec<String>,
        ) -> Result<(), String>
        where
            CD: EntityComponentDirectory,
        {
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

            let label: String = match db.get_entity_component::<Name>(*entity_id) {
                Ok(name) => (**name).clone(),
                Err(_) => "Entity".into(),
            };

            let label = format!("{}:\t{}{}", entity_id, &prefix, label);
            scene_tree_strings.push(label);

            let child_ids: Vec<EntityID>;
            if let Ok(child_entities) = db.get_entity_component::<ChildEntitiesData>(*entity_id) {
                child_ids = child_entities
                    .iter()
                    .filter(|child_id| !db.entity_has_component::<DebugExclude>(child_id))
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

        StoreQuery::<(EntityID, Ref<DebugSceneTree>, RefMut<Vec<String>>)>::iter(
            db.component_store,
        )
        .for_each(|(_, _, mut strings)| *strings = scene_tree_strings.clone());

        Ok(())
    }
}
