use std::borrow::Borrow;

use crate::{
    components::StringListComponent,
    entity_component_system::{SystemError, SystemTrait},
};
use crate::{
    components::{
        ChildEntities, DebugExclude, DebugSceneTree,
        EntityDebugLabels, ParentEntity,
    },
    entity_component_system::{
        system_interface::SystemInterface, ComponentStorage, EntityComponentDirectory, EntityID,
        SystemDebugTrait,
    },
};

#[derive(Debug)]
pub struct SceneTreeDebugSystem;

impl<CS, CD> SystemTrait<CS, CD> for SceneTreeDebugSystem
where
    CS: ComponentStorage,
    CD: EntityComponentDirectory,
{
    fn run(&mut self, db: &mut SystemInterface<CS, CD>) -> Result<(), SystemError>
    where
        CS: ComponentStorage,
        CD: EntityComponentDirectory,
    {
        let mut debug_entities: Vec<EntityID> = db
            .entity_component_directory
            .get_entities_by_predicate(|entity_id| {
                !db.entity_component_directory
                    .entity_has_component::<DebugExclude>(entity_id)
            });
        debug_entities.sort();

        if let Some(entity_debug_entity) =
            db.entity_component_directory
                .get_entity_by_predicate(|entity_id| {
                    db.entity_component_directory
                        .entity_has_component::<EntityDebugLabels>(entity_id)
                })
        {
            let root_entities: Vec<EntityID> = debug_entities
                .iter()
                .filter(|entity_id| {
                    !db.entity_component_directory
                        .entity_has_component::<ParentEntity>(entity_id)
                })
                .copied()
                .collect();

            let mut scene_tree_strings: Vec<String> = Vec::new();

            fn traverse_tree<CS, CD>(
                db: &mut SystemInterface<CS, CD>,
                entity_id: &EntityID,
                entity_debug_entity: EntityID,
                scene_tree_strings: &mut Vec<String>,
                mut padding: Vec<String>,
            ) -> Result<(), String>
            where
                CS: ComponentStorage,
                CD: EntityComponentDirectory,
            {
                let entity_debug_component =
                    db.get_entity_component::<EntityDebugLabels>(entity_debug_entity)?;

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

                let label = entity_debug_component.get_label(entity_id);
                let label = format!("{}:\t{}{}", entity_id, &prefix, label);
                scene_tree_strings.push(label);

                if let Ok(child_entities) =
                    db.get_entity_component::<ChildEntities>(*entity_id)
                {
                    let child_ids: &Vec<EntityID> = child_entities.borrow();
                    let child_ids: Vec<EntityID> = child_ids
                        .iter()
                        .filter(|child_id| {
                            !db.entity_component_directory
                                .entity_has_component::<DebugExclude>(child_id)
                        })
                        .copied()
                        .collect();

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
                        traverse_tree(
                            db,
                            child_entity,
                            entity_debug_entity,
                            scene_tree_strings,
                            padding,
                        )?;
                    }
                }

                Ok(())
            }

            // Populate strings for debug scene tree entities
            for root_entity in &root_entities {
                traverse_tree(
                    db,
                    root_entity,
                    entity_debug_entity,
                    &mut scene_tree_strings,
                    Vec::new(),
                )?;
            }

            let debug_scene_tree_entities = db
                .entity_component_directory
                .get_entities_by_predicate(|entity_id| {
                    db.entity_component_directory
                        .entity_has_component::<DebugSceneTree>(entity_id)
                        && db
                            .entity_component_directory
                            .entity_has_component::<StringListComponent>(entity_id)
                });

            for entity_id in debug_scene_tree_entities {
                db.get_entity_component_mut::<StringListComponent>(entity_id)?
                    .set_data(scene_tree_strings.clone());
            }
        }

        Ok(())
    }
}

impl SystemDebugTrait for SceneTreeDebugSystem {
    fn get_name() -> &'static str {
        "Scene Tree Debug"
    }
}
