use crate::{
    components::{CharComponent, PositionComponent},
    entity_component_system::system_interface::SystemInterface,
    entity_component_system::ComponentStorage,
    entity_component_system::EntityComponentDirectory,
    entity_component_system::SystemDebugTrait,
};
use crate::{
    entity_component_system::{SystemError, SystemTrait},
    primitive_types::IVector2,
};

#[derive(Debug)]
pub struct ASCIIRendererSystem;

impl<CS, CD> SystemTrait<CS, CD> for ASCIIRendererSystem
where
    CS: ComponentStorage,
    CD: EntityComponentDirectory,
{
    fn run(&mut self, db: &mut SystemInterface<CS, CD>) -> Result<(), SystemError>
    where
        CS: ComponentStorage,
        CD: EntityComponentDirectory,
    {
        let entities = db
            .entity_component_directory
            .get_entities_by_predicate(|entity_id| {
                db.entity_component_directory
                    .entity_has_component::<PositionComponent>(entity_id)
                    && db
                        .entity_component_directory
                        .entity_has_component::<CharComponent>(entity_id)
            });

        let mut positions: Vec<(IVector2, char)> = Vec::new();
        for entity_id in entities {
            let position = db
                .get_entity_component::<PositionComponent>(entity_id)?
                .get_position();

            let ascii = *db
                .get_entity_component::<CharComponent>(entity_id)?
                .get_data();

            positions.push((position, ascii))
        }

        for y in 0..10 {
            for x in 0..40 {
                if let Some((_, ascii)) = positions
                    .iter()
                    .find(|(IVector2(pos_x, pos_y), _)| *pos_x == x && *pos_y == y)
                {
                    print!("{}", ascii);
                } else {
                    print!(".")
                }
            }
            println!();
        }

        println!();

        Ok(())
    }
}

impl SystemDebugTrait for ASCIIRendererSystem {
    fn get_name() -> &'static str {
        "ASCII Renderer"
    }
}
