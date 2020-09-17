use antigen::{
    components::GlobalPositionComponent,
    components::ParentEntityComponent,
    components::PositionComponent,
    components::WindowComponent,
    entity_component_system::entity_component_database::ComponentStorage,
    entity_component_system::entity_component_database::EntityComponentDatabase,
    entity_component_system::entity_component_database::EntityComponentDirectory,
    entity_component_system::get_entity_component,
    entity_component_system::get_entity_component_mut,
    entity_component_system::{SystemError, SystemTrait},
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

impl<CS, CD> SystemTrait<CS, CD> for LocalMousePositionSystem
where
    CS: ComponentStorage,
    CD: EntityComponentDirectory,
{
    fn run(&mut self, db: &mut EntityComponentDatabase<CS, CD>) -> Result<(), SystemError>
    where
        CS: ComponentStorage,
        CD: EntityComponentDirectory,
    {
        let mouse_entities = db.get_entities_by_predicate(|entity_id| {
            db.entity_has_component::<PancursesMouseComponent>(entity_id)
        });
        assert!(mouse_entities.len() <= 1);
        let mouse_entity = match mouse_entities.get(0) {
            Some(mouse_entity) => *mouse_entity,
            None => return Err("No mouse entity".into()),
        };
        let mouse_position = get_entity_component::<CS, CD, PancursesMouseComponent>(
            &mut db.component_storage,
            &mut db.entity_component_directory,
            mouse_entity,
        )?
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
                    get_entity_component::<CS, CD, ParentEntityComponent>(
                        &mut db.component_storage,
                        &mut db.entity_component_directory,
                        candidate_id,
                    )
                {
                    candidate_id = parent_entity_component.get_parent_id();
                } else {
                    break;
                }

                if get_entity_component::<CS, CD, WindowComponent>(
                    &mut db.component_storage,
                    &mut db.entity_component_directory,
                    candidate_id,
                )
                .is_ok()
                {
                    let position_component = get_entity_component::<CS, CD, PositionComponent>(
                        &mut db.component_storage,
                        &mut db.entity_component_directory,
                        candidate_id,
                    )?;
                    window_position = position_component.get_position();
                    break;
                }
            }

            let position = match get_entity_component::<CS, CD, GlobalPositionComponent>(
                &mut db.component_storage,
                &mut db.entity_component_directory,
                entity_id,
            ) {
                Ok(global_position_component) => global_position_component.get_global_position(),
                Err(_) => match get_entity_component::<CS, CD, PositionComponent>(
                    &mut db.component_storage,
                    &mut db.entity_component_directory,
                    entity_id,
                ) {
                    Ok(position_component) => position_component.get_position(),
                    Err(err) => return Err(err.into()),
                },
            };

            get_entity_component_mut::<CS, CD, LocalMousePositionComponent>(
                &mut db.component_storage,
                &mut db.entity_component_directory,
                entity_id,
            )?
            .set_local_mouse_position(mouse_position - (window_position + position));
        }

        Ok(())
    }
}
