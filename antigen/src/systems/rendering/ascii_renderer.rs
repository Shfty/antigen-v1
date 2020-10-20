use std::cell::Ref;

use store::StoreQuery;

use crate::{
    components::Position,
    entity_component_system::{
        system_interface::SystemInterface, EntityComponentDirectory, EntityID,
    },
};
use crate::{
    entity_component_system::{SystemError, SystemTrait},
    primitive_types::Vector2I,
};

#[derive(Debug)]
pub struct ASCIIRendererSystem;

impl<CD> SystemTrait<CD> for ASCIIRendererSystem
where
    CD: EntityComponentDirectory,
{
    fn run(&mut self, db: &mut SystemInterface<CD>) -> Result<(), SystemError>
    where
        CD: EntityComponentDirectory,
    {
        let positions: Vec<(Vector2I, char)> =
            StoreQuery::<(EntityID, Ref<Position>, Ref<char>)>::iter(db.component_store)
                .map(|(_, position, char)| (**position, *char))
                .collect();

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
