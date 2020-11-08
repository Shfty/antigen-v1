use crate::primitive_types::HashMap;
use std::cell::Ref;

use store::StoreQuery;

use crate::{
    components::Anchors,
    components::Margins,
    components::Size,
    entity_component_system::EntityID,
    entity_component_system::{SystemError, SystemTrait},
    primitive_types::Vector2I,
};
use crate::{
    components::{ParentEntity, Position},
    entity_component_system::ComponentStore,
};

type ReadAnchorsEntity<'a> = (
    EntityID,
    Ref<'a, Anchors>,
    Ref<'a, Position>,
    Ref<'a, ParentEntity>,
);

#[derive(Debug)]
pub struct AnchorsMargins;

impl SystemTrait for AnchorsMargins {
    fn run(&mut self, db: &mut ComponentStore) -> Result<(), SystemError> {
        // Sort into a HashMap based on tree depth
        let mut tree_depth_entities: HashMap<i64, Vec<EntityID>> = HashMap::default();

        StoreQuery::<ReadAnchorsEntity>::iter(db.as_ref()).for_each(
            |(entity_id, _, _, parent_entity)| {
                let parent_id: EntityID = **parent_entity;

                let mut candidate_id = parent_id;
                let mut depth = 0i64;
                loop {
                    depth += 1;
                    match db.get::<ParentEntity>(&candidate_id) {
                        Some(parent_entity_component) => {
                            candidate_id = **parent_entity_component;
                        }
                        None => break,
                    }
                }

                match tree_depth_entities.get_mut(&depth) {
                    Some(tree_depth) => {
                        tree_depth.push(entity_id);
                    }
                    None => {
                        tree_depth_entities.insert(depth, vec![entity_id]);
                    }
                };
            },
        );

        // Convert HashMap into a vector, starting at the root layer and moving down
        let mut tree_depth_keys: Vec<i64> = tree_depth_entities.keys().copied().collect();
        tree_depth_keys.sort();

        // Update position and size based on anchors
        for entity_id in tree_depth_keys
            .into_iter()
            .flat_map(|key| tree_depth_entities.get(&key).into_iter().flatten())
        {
            let parent_id: EntityID = **db.get::<ParentEntity>(entity_id).unwrap();

            let Vector2I(parent_width, parent_height) = **db.get::<Size>(&parent_id).unwrap();

            let (anchor_left, anchor_right, anchor_top, anchor_bottom) =
                db.get::<Anchors>(entity_id).unwrap().get_anchors();

            let (margin_left, margin_right, margin_top, margin_bottom) =
                match db.get::<Margins>(entity_id) {
                    Some(margins_component) => margins_component.get_margins(),
                    None => (0, 0, 0, 0),
                };

            let x = margin_left + (parent_width as f32 * anchor_left).floor() as i64;
            let y = margin_top + (parent_height as f32 * anchor_top).floor() as i64;

            **db.get_mut::<Position>(entity_id).unwrap() = Vector2I(x, y);

            let width = (parent_width as f32 * (anchor_right - anchor_left)).ceil() as i64
                - (margin_right + margin_left);
            let width = std::cmp::max(width, 0);

            let height = (parent_height as f32 * (anchor_bottom - anchor_top)).ceil() as i64
                - (margin_bottom + margin_top);
            let height = std::cmp::max(height, 0);

            **db.get_mut::<Size>(entity_id).unwrap() = Vector2I(width, height);
        }

        Ok(())
    }
}
