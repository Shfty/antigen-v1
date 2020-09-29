use crate::{
    components::Position, entity_component_system::system_interface::SystemInterface,
    entity_component_system::ComponentStorage, entity_component_system::EntityComponentDirectory,
};
use crate::{
    entity_component_system::{SystemError, SystemTrait},
    primitive_types::Vector2I,
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
                    .entity_has_component::<Position>(entity_id)
                    && db
                        .entity_component_directory
                        .entity_has_component::<char>(entity_id)
            });

        let mut positions: Vec<(Vector2I, char)> = Vec::new();
        for entity_id in entities {
            let position: Vector2I = (*db.get_entity_component::<Position>(entity_id)?).into();

            let ascii = *db.get_entity_component::<char>(entity_id)?;

            positions.push((position, ascii))
        }

        for y in 0..10 {
            for x in 0..40 {
                if let Some((_, ascii)) = positions
                    .iter()
                    .find(|(Vector2I(pos_x, pos_y), _)| *pos_x == x && *pos_y == y)
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
