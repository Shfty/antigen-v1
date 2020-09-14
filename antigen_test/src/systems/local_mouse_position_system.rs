use antigen::{
    components::GlobalPositionComponent,
    components::ParentEntityComponent,
    components::PositionComponent,
    components::WindowComponent,
    ecs::{EntityComponentDatabase, SystemError, SystemTrait},
    primitive_types::IVector2,
};

use crate::components::{
    local_mouse_position_component::LocalMousePositionComponent,
    pancurses_mouse_component::PancursesMouseComponent,
};

#[derive(Debug)]
pub struct LocalMousePositionSystem;

impl Default for LocalMousePositionSystem {
    fn default() -> Self {
        LocalMousePositionSystem
    }
}

impl LocalMousePositionSystem {
    pub fn new() -> Self {
        LocalMousePositionSystem::default()
    }
}

impl<T> SystemTrait<T> for LocalMousePositionSystem
where
    T: EntityComponentDatabase,
{
    fn run(&mut self, db: &mut T) -> Result<(), SystemError>
    where
        T: EntityComponentDatabase,
    {
        let mouse_entities = db.get_entities_by_predicate(|entity_id| {
            db.entity_has_component::<PancursesMouseComponent>(entity_id)
        });
        assert!(mouse_entities.len() <= 1);
        let mouse_entity = match mouse_entities.get(0) {
            Some(mouse_entity) => *mouse_entity,
            None => return Err("No mouse entity".into()),
        };
        let mouse_position = db
            .get_entity_component::<PancursesMouseComponent>(mouse_entity)?
            .get_position();

        let entities = db.get_entities_by_predicate(|entity_id| {
            db.entity_has_component::<LocalMousePositionComponent>(entity_id)
                && db.entity_has_component::<PositionComponent>(entity_id)
        });

        for entity_id in entities {
            let mut candidate_id = entity_id;
            let mut window_position = IVector2::default();
            loop {
                if let Ok(parent_entity_component) =
                    db.get_entity_component::<ParentEntityComponent>(candidate_id)
                {
                    candidate_id = parent_entity_component.get_parent_id();
                } else {
                    break;
                }

                if db
                    .get_entity_component::<WindowComponent>(candidate_id)
                    .is_ok()
                {
                    let position_component =
                        db.get_entity_component::<PositionComponent>(candidate_id)?;
                    window_position = position_component.get_position();
                    break;
                }
            }

            let position = match db.get_entity_component::<GlobalPositionComponent>(entity_id) {
                Ok(global_position_component) => global_position_component.get_global_position(),
                Err(_) => match db.get_entity_component::<PositionComponent>(entity_id) {
                    Ok(position_component) => position_component.get_position(),
                    Err(err) => return Err(err.into()),
                },
            };

            db.get_entity_component_mut::<LocalMousePositionComponent>(entity_id)?
                .set_local_mouse_position(mouse_position - (window_position + position));
        }

        Ok(())
    }
}
