use antigen::{
    components::VelocityComponent,
    ecs::{SystemTrait, ECS},
};

pub struct InputSystem {
    input_buffer: Vec<pancurses::Input>,
}

impl InputSystem {
    pub fn new() -> Self {
        InputSystem {
            input_buffer: Vec::new(),
        }
    }

    pub fn set_input_buffer(&mut self, buffer: &[pancurses::Input]) {
        self.input_buffer
            .resize(buffer.len(), pancurses::Input::Unknown(0));
        self.input_buffer.copy_from_slice(buffer)
    }
}

impl SystemTrait for InputSystem {
    fn run(&mut self, ecs: &mut ECS) -> Result<(), String> {
        let mut move_input: (i64, i64) = (0, 0);
        while let Some(input) = self.input_buffer.pop() {
            match input {
                pancurses::Input::KeyLeft => move_input.0 -= 1,
                pancurses::Input::KeyRight => move_input.0 += 1,
                pancurses::Input::KeyUp => move_input.1 -= 1,
                pancurses::Input::KeyDown => move_input.1 += 1,
                _ => (),
            }
        }

        move_input.0 = std::cmp::min(std::cmp::max(move_input.0, -1), 1);
        move_input.1 = std::cmp::min(std::cmp::max(move_input.1, -1), 1);

        let entities = ecs
            .build_entity_query()
            .component::<VelocityComponent>()
            .finish();

        for entity_id in entities {
            let velocity_component = ecs.get_entity_component::<VelocityComponent>(entity_id)?;
            velocity_component.x = move_input.0;
            velocity_component.y = move_input.1;
        }

        Ok(())
    }
}
