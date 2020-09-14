use std::collections::HashMap;

use crate::components::{ParentEntityComponent, PositionComponent};
use crate::{
    components::AnchorsComponent,
    components::MarginsComponent,
    components::SizeComponent,
    ecs::EntityID,
    ecs::{EntityComponentDatabase, SystemError, SystemTrait},
    primitive_types::IVector2,
};

#[derive(Debug)]
pub struct AnchorsMarginsSystem;

impl Default for AnchorsMarginsSystem {
    fn default() -> Self {
        AnchorsMarginsSystem
    }
}

impl AnchorsMarginsSystem {
    pub fn new() -> Self {
        AnchorsMarginsSystem::default()
    }
}

impl<T> SystemTrait<T> for AnchorsMarginsSystem
where
    T: EntityComponentDatabase,
{
    fn run(&mut self, db: &mut T) -> Result<(), SystemError>
    where
        T: EntityComponentDatabase,
    {
        // Fetch anchor entities
        let anchor_entities = db.get_entities_by_predicate(|entity_id| {
            db.entity_has_component::<AnchorsComponent>(entity_id)
                && db.entity_has_component::<PositionComponent>(entity_id)
                && db.entity_has_component::<ParentEntityComponent>(entity_id)
        });

        // Sort into a HashMap based on tree depth
        let mut tree_depth_entities: HashMap<i64, Vec<EntityID>> = HashMap::new();

        for entity_id in &anchor_entities {
            let parent_id = db
                .get_entity_component::<ParentEntityComponent>(*entity_id)?
                .get_parent_id();

            let mut candidate_id = parent_id;
            let mut depth = 0i64;
            loop {
                depth += 1;
                match db.get_entity_component::<ParentEntityComponent>(candidate_id) {
                    Ok(parent_entity_component) => {
                        candidate_id = parent_entity_component.get_parent_id();
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
            let parent_id = db
                .get_entity_component::<ParentEntityComponent>(entity_id)?
                .get_parent_id();

            let parent_position_component =
                db.get_entity_component::<PositionComponent>(parent_id)?;
            let IVector2(parent_pos_x, parent_pos_y) = parent_position_component.get_position();

            let IVector2(parent_width, parent_height) = db
                .get_entity_component::<SizeComponent>(parent_id)?
                .get_size();

            let (anchor_left, anchor_right, anchor_top, anchor_bottom) = db.get_entity_component::<AnchorsComponent>(entity_id)?.get_anchors();

            let (margin_left, margin_right, margin_top, margin_bottom) =
                match db.get_entity_component::<MarginsComponent>(entity_id) {
                    Ok(margins_component) => margins_component.get_margins(),
                    Err(_) => (0, 0, 0, 0),
                };

            let x = margin_left + parent_pos_x + (parent_width as f32 * anchor_left).floor() as i64;
            let y = margin_top + parent_pos_y + (parent_height as f32 * anchor_top).floor() as i64;

            db.get_entity_component_mut::<PositionComponent>(entity_id)?
                .set_position(IVector2(x, y));

            let width = (parent_width as f32 * (anchor_right - anchor_left)).ceil() as i64
                - (margin_right + margin_left);
            let width = std::cmp::max(width, 0);

            let height = (parent_height as f32 * (anchor_bottom - anchor_top)).ceil() as i64
                - (margin_bottom + margin_top);
            let height = std::cmp::max(height, 0);

            db.get_entity_component_mut::<SizeComponent>(entity_id)?
                .set_size(IVector2(width, height));
        }

        Ok(())
    }
}
