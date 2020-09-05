mod components;
mod systems;

use std::{collections::HashMap, time::Duration};

use antigen::components::{
    CharComponent, PositionComponent, StringSliceComponent, VelocityComponent,
};
use antigen::ecs::{AssemblageID, SystemTrait, ECS};
use antigen::systems::PositionIntegratorSystem;

use components::pancurses_color_component::{
    PancursesColor, PancursesColorPair, PancursesColorPairComponent,
};
use systems::{InputSystem, PancursesRendererSystem};

// TODO: More in-depth query setup
//       Example: Fetch all entities with Component A and (Component B or Component C)
//       Concrete case: Pancurses renderer currently has to fetch all objects with positions, then filter by CharComponent / StringSliceComponent locally
//       Use predicate closures + ECS::entity_has_component?
// TODO: Pancurses-compatible UI components
// TODO: Debug menu
// TODO: Refactor TickMoveComponent into TimerComponent (useless without some form of event system)

#[derive(Eq, PartialEq, Hash)]
enum Assemblage {
    CharEntity = 0,
    StringEntity = 1,
}

// Main Logic
fn main() {
    let mut ecs = ECS::new();

    register_components(&mut ecs);
    let assemblages = register_assemblages(&mut ecs);
    catch_error(ecs.assemble_entity(assemblages[&Assemblage::CharEntity], "Test Char Entity"));
    catch_error(ecs.assemble_entity(assemblages[&Assemblage::StringEntity], "Test String Entity"));

    let mut input_system = InputSystem::new();
    let mut position_integrator_system = PositionIntegratorSystem::new();
    let mut pancurses_renderer_system =
        PancursesRendererSystem::new(128, 32, '.', PancursesColorPair::default());

    pancurses_renderer_system.initialize();

    loop {
        let input_buffer = pancurses_renderer_system.get_input();
        if input_buffer
            .iter()
            .any(|input| *input == pancurses::Input::Character('\u{1b}'))
        {
            break;
        }

        input_system.set_input_buffer(&input_buffer);
        catch_error(input_system.run(&mut ecs));
        catch_error(position_integrator_system.run(&mut ecs));
        catch_error(pancurses_renderer_system.run(&mut ecs));

        //println!("{:#?}", ecs);

        std::thread::sleep(Duration::from_secs_f32(1.0 / 60.0));
    }

    pancurses_renderer_system.finalize();
}

fn register_components(ecs: &mut ECS) {
    ecs.register_component::<PositionComponent>("Position", "2D cartesian position");
    ecs.register_component::<VelocityComponent>("Velocity", "2D cartesian velocity");
    ecs.register_component::<CharComponent>("Char", "ASCII character");
    ecs.register_component::<StringSliceComponent>("String", "Text string");
    ecs.register_component::<PancursesColorPairComponent>(
        "Pancurses Color",
        "RGB color for use with Pancurses",
    );
}

fn register_assemblages(ecs: &mut ECS) -> HashMap<Assemblage, AssemblageID> {
    vec![
        (
            Assemblage::CharEntity,
            ecs.build_assemblage("Char Entity", "ASCII character with position and velocity")
                .component(PositionComponent::new(1, 1))
                .component(VelocityComponent::new(1, 1))
                .component(CharComponent::new('@'))
                .component(PancursesColorPairComponent::new(PancursesColorPair::new(
                    PancursesColor::new(753, 753, 753),
                    PancursesColor::new(753, 0, 753),
                )))
                .finish(),
        ),
        (
            Assemblage::StringEntity,
            ecs.build_assemblage("String Entity", "ASCII string with position and velocity")
                .component(PositionComponent::new(1, 3))
                .component(VelocityComponent::new(1, 1))
                .component(StringSliceComponent::new("Testing One Two Three"))
                .component(PancursesColorPairComponent::new(PancursesColorPair::new(
                    PancursesColor::new(753, 753, 753),
                    PancursesColor::new(753, 0, 753),
                )))
                .finish(),
        ),
    ]
    .into_iter()
    .collect()
}

fn catch_error<T>(result: Result<T, String>) -> T {
    if let Err(err) = &result {
        handle_error(err);
    }

    result.unwrap()
}

fn handle_error(err: &str) {
    eprintln!("{}", err);
    std::process::exit(1);
}
