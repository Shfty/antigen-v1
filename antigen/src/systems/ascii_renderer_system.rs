use crate::{
    components::{CharComponent, PositionComponent},
    ecs::EntityComponentSystemDebug,
};
use crate::{
    ecs::{EntityComponentSystem, SystemEvent, SystemTrait},
    primitive_types::IVector2,
};

#[derive(Debug)]
pub struct ASCIIRendererSystem;

impl<T> SystemTrait<T> for ASCIIRendererSystem
where
    T: EntityComponentSystem + EntityComponentSystemDebug,
{
    fn run(&mut self, ecs: &mut T) -> Result<SystemEvent, String>
    where
        T: EntityComponentSystem + EntityComponentSystemDebug,
    {
        let entities = ecs.get_entities_by_predicate(|entity_id| {
            ecs.entity_has_component::<PositionComponent>(entity_id)
                && ecs.entity_has_component::<CharComponent>(entity_id)
        });

        let mut positions: Vec<(IVector2, char)> = Vec::new();
        for entity_id in entities {
            let ascii_component = ecs.get_entity_component::<CharComponent>(entity_id)?;
            let ascii = ascii_component.data;

            let position_component = ecs.get_entity_component::<PositionComponent>(entity_id)?;
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
