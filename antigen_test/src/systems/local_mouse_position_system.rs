use antigen::{
    components::GlobalPositionComponent,
    components::ParentEntityComponent,
    components::PositionComponent,
    components::WindowComponent,
    entity_component_system::system_interface::SystemInterface,
    entity_component_system::ComponentStorage,
    entity_component_system::EntityComponentDirectory,
    entity_component_system::{SystemError, SystemTrait},
    primitive_types::IVector2,
entity_component_system::SystemDebugTrait};

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

impl<CS, CD> SystemTrait<CS, CD> for LocalMousePositionSystem
where
    CS: ComponentStorage,
    CD: EntityComponentDirectory,
{
    fn run(&mut self, db: &mut SystemInterface<CS, CD>) -> Result<(), SystemError>
    where
        CS: ComponentStorage,
        CD: EntityComponentDirectory,
    {
        let mouse_entities = db
            .entity_component_directory
            .get_entities_by_predicate(|entity_id| {
                db.entity_component_directory
                    .entity_has_component::<PancursesMouseComponent>(entity_id)
            });
        assert!(mouse_entities.len() <= 1);
        let mouse_entity = match mouse_entities.get(0) {
            Some(mouse_entity) => *mouse_entity,
            None => return Err("No mouse entity".into()),
        };
        let mouse_position = db
            .get_entity_component::<PancursesMouseComponent>(mouse_entity)?
            .get_position();

        let entities = db
            .entity_component_directory
            .get_entities_by_predicate(|entity_id| {
                db.entity_component_directory
                    .entity_has_component::<LocalMousePositionComponent>(entity_id)
                    && db
                        .entity_component_directory
                        .entity_has_component::<PositionComponent>(entity_id)
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

impl SystemDebugTrait for LocalMousePositionSystem {
    fn get_name() -> &'static str {
        "Local Mouse Position"
    }
}
