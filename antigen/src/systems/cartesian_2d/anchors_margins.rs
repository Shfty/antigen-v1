use std::collections::HashMap;

use crate::{
    components::Anchors,
    components::Margins,
    components::Size,
    entity_component_system::system_interface::SystemInterface,
    entity_component_system::EntityID,
    entity_component_system::{SystemError, SystemTrait},
    primitive_types::Vector2I,
};
use crate::{
    components::{ParentEntity, Position},
    entity_component_system::ComponentStorage,
    entity_component_system::EntityComponentDirectory,
};

#[derive(Debug)]
pub struct AnchorsMargins;

impl Default for AnchorsMargins {
    fn default() -> Self {
        AnchorsMargins
    }
}

impl AnchorsMargins {
    pub fn new() -> Self {
        AnchorsMargins::default()
    }
}

impl<CS, CD> SystemTrait<CS, CD> for AnchorsMargins
where
    CS: ComponentStorage,
    CD: EntityComponentDirectory,
{
    fn run(&mut self, db: &mut SystemInterface<CS, CD>) -> Result<(), SystemError>
    where
        CD: EntityComponentDirectory,
    {
        // Fetch anchor entities
        let anchor_entities =
            db.entity_component_directory
                .get_entities_by_predicate(|entity_id| {
                    db.entity_component_directory
                        .entity_has_component::<Anchors>(entity_id)
                        && db
                            .entity_component_directory
                            .entity_has_component::<Position>(entity_id)
                        && db
                            .entity_component_directory
                            .entity_has_component::<ParentEntity>(entity_id)
                });

        // Sort into a HashMap based on tree depth
        let mut tree_depth_entities: HashMap<i64, Vec<EntityID>> = HashMap::new();

        for entity_id in &anchor_entities {
            let parent_id: EntityID =
                (*db.get_entity_component::<ParentEntity>(*entity_id)?).into();

            let mut candidate_id = parent_id;
            let mut depth = 0i64;
            loop {
                depth += 1;
                match db.get_entity_component::<ParentEntity>(candidate_id) {
                    Ok(parent_entity_component) => {
                        candidate_id = (*parent_entity_component).into();
                    }
                    Err(_) => break,
                }
            }

            match tree_depth_entities.get_mut(&depth) {
                Some(tree_depth) => {
                    tree_depth.push(*entity_id);
                }
                None => {
                    tree_depth_entities.insert(depth, vec![*entity_id]);
                }
            };
        }

        // Convert HashMap into a vector, starting at the root layer and moving down
        let mut anchor_entities: Vec<EntityID> = Vec::new();
        let mut tree_depth_keys: Vec<i64> = tree_depth_entities.keys().copied().collect();
        tree_depth_keys.sort();
        for i in tree_depth_keys {
            if let Some(tree_depth_entities) = tree_depth_entities.get_mut(&i) {
                anchor_entities.append(tree_depth_entities);
            }
        }

        // Update position and size based on anchors
        for entity_id in anchor_entities {
            let parent_id: EntityID = (*db.get_entity_component::<ParentEntity>(entity_id)?).into();

            let parent_position = db.get_entity_component::<Position>(parent_id)?;
            let parent_position = *parent_position;
            let Vector2I(parent_pos_x, parent_pos_y) = parent_position.into();

            let Vector2I(parent_width, parent_height) =
                (*db.get_entity_component::<Size>(parent_id)?).into();

            let (anchor_left, anchor_right, anchor_top, anchor_bottom) =
                db.get_entity_component::<Anchors>(entity_id)?.get_anchors();

            let (margin_left, margin_right, margin_top, margin_bottom) =
                match db.get_entity_component::<Margins>(entity_id) {
                    Ok(margins_component) => margins_component.get_margins(),
                    Err(_) => (0, 0, 0, 0),
                };

            let x = margin_left + parent_pos_x + (parent_width as f32 * anchor_left).floor() as i64;
            let y = margin_top + parent_pos_y + (parent_height as f32 * anchor_top).floor() as i64;

            *db.get_entity_component_mut::<Position>(entity_id)? = Vector2I(x, y).into();

            let width = (parent_width as f32 * (anchor_right - anchor_left)).ceil() as i64
                - (margin_right + margin_left);
            let width = std::cmp::max(width, 0);

            let height = (parent_height as f32 * (anchor_bottom - anchor_top)).ceil() as i64
                - (margin_bottom + margin_top);
            let height = std::cmp::max(height, 0);

            *db.get_entity_component_mut::<Size>(entity_id)? = Vector2I(width, height).into();
        }

        Ok(())
    }
}
