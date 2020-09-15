use crate::{
    components::{CharComponent, PositionComponent},
    entity_component_system::entity_component_database::EntityComponentDatabase,
    entity_component_system::ComponentStorage,
};
use crate::{
    entity_component_system::{EntityComponentDirectory, SystemError, SystemTrait},
    primitive_types::IVector2,
};

#[derive(Debug)]
pub struct ASCIIRendererSystem;

impl<S, D> SystemTrait<S, D> for ASCIIRendererSystem
where
    S: ComponentStorage,
    D: EntityComponentDirectory,
{
    fn run(&mut self, db: &mut EntityComponentDatabase<S, D>) -> Result<(), SystemError>
    where
        S: ComponentStorage,
        D: EntityComponentDirectory,
    {
        let entities = db.get_entities_by_predicate(|entity_id| {
            db.entity_has_component::<PositionComponent>(entity_id)
                && db.entity_has_component::<CharComponent>(entity_id)
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
