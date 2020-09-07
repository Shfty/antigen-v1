use super::ECS;

pub trait SystemTrait<T: ECS> {
    fn run(&mut self, ecs: &mut T) -> Result<(), String>;
}
