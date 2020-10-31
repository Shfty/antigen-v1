use std::cell::Ref;

use store::StoreQuery;

use crate::{
    components::Position,
    entity_component_system::{ComponentStore, EntityID},
};
use crate::{
    entity_component_system::{SystemError, SystemTrait},
    primitive_types::Vector2I,
};

#[derive(Debug)]
pub struct ASCIIRendererSystem;

impl SystemTrait for ASCIIRendererSystem {
    fn run(&mut self, db: &mut ComponentStore) -> Result<(), SystemError> {
        let positions: Vec<(Vector2I, char)> =
            StoreQuery::<(EntityID, Ref<Position>, Ref<char>)>::iter(db.as_ref())
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
