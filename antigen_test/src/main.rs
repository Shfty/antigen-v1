mod components;
mod scenes;
mod systems;

use std::time::Duration;

use antigen::{
    core::profiler::Profiler,
    entity_component_system::{EntityComponentSystem, SystemError},
};

fn main() -> Result<(), SystemError> {
    // Create and populate ECS
    let mut ecs = EntityComponentSystem::default();
    scenes::antigen_debug_scene::system_assembler(Default::default()).finish(&mut ecs);
    scenes::antigen_debug_scene::entity_assembler(Default::default())
        .finish(ecs.get_component_store());

    // Run main loop
    let frame_time_target = Duration::from_secs_f32(1.0 / 60.0);
    loop {
        let main_loop_profiler = Profiler::start();

        if let Err(err) = ecs.run() {
            match err {
                SystemError::Err(_) => return Err(err),
                SystemError::Quit => return Ok(()),
            }
        }

        // Sleep if framerate target is exceeded - prevents deadlock when pancurses stops being able to poll input after window close
        let delta = main_loop_profiler.finish();
        if delta < frame_time_target {
            let delta = frame_time_target - delta;
            std::thread::sleep(delta);
        }
    }
}
