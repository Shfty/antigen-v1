use super::ECS;

pub trait SystemTrait {
    fn run(&mut self, ecs: &mut ECS) -> Result<(), String>;
}
