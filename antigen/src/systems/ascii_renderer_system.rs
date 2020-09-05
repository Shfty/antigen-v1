use crate::components::{CharComponent, PositionComponent};
use crate::ecs::{SystemTrait, ECS};

#[derive(Debug)]
pub struct ASCIIRendererSystem;

impl SystemTrait for ASCIIRendererSystem {
    fn run(&mut self, ecs: &mut ECS) -> Result<(), String> {
        let entities = ecs
            .build_entity_query()
            .component::<PositionComponent>()
            .component::<CharComponent>()
            .finish();

        let mut positions: Vec<(i64, i64, char)> = Vec::new();
        for entity_id in entities {
            let ascii_component = ecs.get_entity_component::<CharComponent>(entity_id)?;
            let ascii = ascii_component.data;

            let position_component = ecs.get_entity_component::<PositionComponent>(entity_id)?;
            positions.push((position_component.x, position_component.y, ascii))
        }

        for y in 0..10 {
            for x in 0..40 {
                if let Some((_, _, ascii)) = positions
                    .iter()
                    .find(|(pos_x, pos_y, _)| *pos_x == x && *pos_y == y)
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
