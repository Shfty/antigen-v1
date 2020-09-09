use crate::{
    components::{CharComponent, PositionComponent},
    ecs::EntityComponentDatabaseDebug,
};
use crate::{
    ecs::{EntityComponentDatabase, SystemEvent, SystemTrait},
    primitive_types::IVector2,
};

#[derive(Debug)]
pub struct ASCIIRendererSystem;

impl<T> SystemTrait<T> for ASCIIRendererSystem
where
    T: EntityComponentDatabase + EntityComponentDatabaseDebug,
{
    fn run(&mut self, db: &mut T) -> Result<SystemEvent, String>
    where
        T: EntityComponentDatabase + EntityComponentDatabaseDebug,
    {
        let entities = db.get_entities_by_predicate(|entity_id| {
            db.entity_has_component::<PositionComponent>(entity_id)
                && db.entity_has_component::<CharComponent>(entity_id)
        });

        let mut positions: Vec<(IVector2, char)> = Vec::new();
        for entity_id in entities {
            let ascii_component = db.get_entity_component::<CharComponent>(entity_id)?;
            let ascii = ascii_component.data;

            let position_component = db.get_entity_component::<PositionComponent>(entity_id)?;
            positions.push((position_component.data, ascii))
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

        Ok(SystemEvent::None)
    }
}
