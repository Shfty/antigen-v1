mod components;
mod pancurses_color;
mod systems;

mod unboxed_test;

use std::{
    collections::HashMap,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use antigen::{
    components::{
        CharComponent, GlobalPositionComponent, IntRangeComponent, ParentEntityComponent,
        PositionComponent, SizeComponent, StringComponent, VelocityComponent,
    },
    ecs::{
        components::{DebugData, ECSDebugComponent},
        systems::ECSDebugSystem,
        AssemblageID, SingleThreadedECS, EntityID, SystemTrait, ECS,
    },
    primitive_types::IVector2,
    systems::{GlobalPositionSystem, PositionIntegratorSystem},
};

use components::{
    pancurses_color_pair_component::PancursesColorPairComponent,
    pancurses_control_component::{ControlData, PancursesControlComponent},
    pancurses_input_buffer_component::PancursesInputBufferComponent,
    pancurses_prev_next_input_component::PancursesPrevNextInputComponent,
    pancurses_window_component::{PancursesWindowComponent, WindowID},
};
use pancurses_color::{PancursesColor, PancursesColorPair};
use systems::{
    DebugTabSystem, InputVelocitySystem, PancursesInputSystem, PancursesPrevNextInputSystem,
    PancursesRendererSystem,
};

// TODO: Pancurses-compatible UI controls
//       - List
//       - List item
// TODO: Debug menu
// TODO: Profiler singleton
// TODO: Profiler menu
// TODO: Better automation for system execution
//       How to account for systems that need to take input from the outside world during the main loop?
//       Currently splitting them out as special cases, but that doesn't seem like good practice
//       Could add input as a new system event? Would need to be generic, or some sort of antigen-specific input type
// TODO: Clone trait boundary should only be enforced when trying to register a component as part of an assemblage

#[derive(Eq, PartialEq, Hash)]
enum EntityAssemblage {
    Player = 0,
    StringControl = 1,
    RectControl = 2,
    BorderControl = 3,
}

// Main Logic
fn main() {
    if let Err(err) = main_internal() {
        eprintln!("{}", err);
        std::process::exit(1);
    }
}

fn create_string_control(
    ecs: &mut impl ECS,
    assemblage_id: AssemblageID,
    label: &str,
    text: &str,
    window_id: WindowID,
    (x, y): (i64, i64),
) -> Result<EntityID, String> {
    let entity_id = ecs.assemble_entity(assemblage_id, label)?;

    let debug_title_component = ecs.get_entity_component::<PancursesControlComponent>(entity_id)?;
    debug_title_component.control_data = ControlData::String;
    debug_title_component.window_id = window_id;

    let string_component = ecs.get_entity_component::<StringComponent>(entity_id)?;
    string_component.data = text.into();

    let position_component = ecs.get_entity_component::<PositionComponent>(entity_id)?;
    let IVector2(pos_x, pos_y) = &mut position_component.data;
    *pos_x = x;
    *pos_y = y;

    Ok(entity_id)
}

struct Profiler {
    name: String,
    start_ts: Duration,
}

impl Profiler {
    fn start(name: &str) -> Profiler {
        println!("{} start", &name);

        Profiler {
            name: name.into(),
            start_ts: Self::get_now(),
        }
    }

    fn get_now() -> Duration {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
    }

    fn finish(self) -> Duration {
        let delta = Self::get_now() - self.start_ts;
        println!(
            "{} finish. Took {}ms / {}ns / {}us",
            self.name,
            delta.as_millis(),
            delta.as_micros(),
            delta.as_nanos()
        );
        delta
    }
}

fn main_internal() -> Result<(), String> {
    let mut ecs = SingleThreadedECS::new();
    let assemblages = register_assemblages(&mut ecs);

    // Create Main Window
    let main_window_entity = ecs.create_entity("Main Window");
    ecs.add_component_to_entity(main_window_entity, PositionComponent::default())?;
    ecs.add_component_to_entity(main_window_entity, SizeComponent::new(IVector2(128, 32)))?;
    ecs.add_component_to_entity(main_window_entity, PancursesWindowComponent::new(0))?;

    // Create Game Window
    let game_window_entity = ecs.create_entity("Game Window");
    ecs.add_component_to_entity(game_window_entity, PositionComponent::default())?;
    ecs.add_component_to_entity(game_window_entity, SizeComponent::new(IVector2(64, 32)))?;
    ecs.add_component_to_entity(game_window_entity, PancursesWindowComponent::new(2))?;

    // Create Test Player
    let test_player_entity =
        ecs.assemble_entity(assemblages[&EntityAssemblage::Player], "Test Player")?;
    if let Ok(pancurses_control_component) =
        ecs.get_entity_component::<PancursesControlComponent>(test_player_entity)
    {
        pancurses_control_component.window_id = 2;
    }

    // Create Test String
    let test_string_entity = ecs.assemble_entity(
        assemblages[&EntityAssemblage::StringControl],
        "Test String Control",
    )?;
    ecs.add_component_to_entity(
        test_string_entity,
        ParentEntityComponent::new(test_player_entity),
    )?;
    ecs.add_component_to_entity(test_string_entity, GlobalPositionComponent::default())?;
    if let Ok(position_component) =
        ecs.get_entity_component::<PositionComponent>(test_string_entity)
    {
        position_component.data = IVector2(1, 1);
    }
    if let Ok(string_component) = ecs.get_entity_component::<StringComponent>(test_string_entity) {
        string_component.data = "Testing One Two Three".into();
    }
    if let Ok(pancurses_control_component) =
        ecs.get_entity_component::<PancursesControlComponent>(test_string_entity)
    {
        pancurses_control_component.window_id = 2;
    }

    // Create Test Rect
    let test_rect_entity = ecs.assemble_entity(
        assemblages[&EntityAssemblage::RectControl],
        "Test Rect Control",
    )?;
    if let Ok(position_component) = ecs.get_entity_component::<PositionComponent>(test_rect_entity)
    {
        position_component.data = IVector2(1, 5);
    }
    if let Ok(size_component) = ecs.get_entity_component::<SizeComponent>(test_rect_entity) {
        size_component.data = IVector2(20, 5);
    }
    if let Ok(pancurses_control_component) =
        ecs.get_entity_component::<PancursesControlComponent>(test_rect_entity)
    {
        pancurses_control_component.window_id = 2;
    }

    // Create Debug Window
    let debug_window_entity = ecs.create_entity("Debug Window");
    ecs.add_component_to_entity(debug_window_entity, PositionComponent::new(IVector2(64, 0)))?;
    ecs.add_component_to_entity(debug_window_entity, SizeComponent::new(IVector2(64, 32)))?;
    ecs.add_component_to_entity(debug_window_entity, PancursesWindowComponent::new(1))?;

    let debug_window_border_entity = ecs.assemble_entity(
        assemblages[&EntityAssemblage::BorderControl],
        "Debug Window Border",
    )?;
    if let Ok(pancurses_control_component) =
        ecs.get_entity_component::<PancursesControlComponent>(debug_window_border_entity)
    {
        pancurses_control_component.window_id = 1;
    }
    if let Ok(size_component) =
        ecs.get_entity_component::<SizeComponent>(debug_window_border_entity)
    {
        size_component.data = IVector2(64, 32);
    }

    // Create Debug Window Title
    create_string_control(
        &mut ecs,
        assemblages[&EntityAssemblage::StringControl],
        "Debug Title",
        "Debug",
        1,
        (1, 1),
    )?;

    // Create Debug Window String List
    let debug_list_entity = create_string_control(
        &mut ecs,
        assemblages[&EntityAssemblage::StringControl],
        "Debug List",
        "List",
        1,
        (1, 2),
    )?;

    ecs.add_component_to_entity(
        debug_list_entity,
        ECSDebugComponent::new(DebugData::Components),
    )?;
    ecs.add_component_to_entity(
        debug_list_entity,
        PancursesPrevNextInputComponent::new(
            pancurses::Input::KeyPPage,
            pancurses::Input::KeyNPage,
        ),
    )?;
    ecs.add_component_to_entity(debug_list_entity, PancursesInputBufferComponent::default())?;
    ecs.add_component_to_entity(debug_list_entity, IntRangeComponent::new(0..5))?;

    // Create systems
    let mut pancurses_input_system = PancursesInputSystem::new();
    let mut ui_tab_input_system = PancursesPrevNextInputSystem::new();
    let mut input_velocity_system = InputVelocitySystem::new();
    let mut position_integrator_system = PositionIntegratorSystem::new();
    let mut global_position_system = GlobalPositionSystem::new();
    let mut debug_tab_system = DebugTabSystem::new();
    let mut ecs_debug_system = ECSDebugSystem::new();
    let mut pancurses_renderer_system = PancursesRendererSystem::new(1);

    let mut systems: Vec<(&str, &mut dyn SystemTrait<SingleThreadedECS>)> = vec![
        ("UI Tab Input System", &mut ui_tab_input_system),
        ("Input Velocity System", &mut input_velocity_system),
        (
            "Position Integrator System",
            &mut position_integrator_system,
        ),
        ("Global Position System", &mut global_position_system),
        ("Debug Tab System", &mut debug_tab_system),
        ("ECS Debug System", &mut ecs_debug_system),
    ];

    // Main loop
    loop {
        let main_loop_profiler = Profiler::start("Main Loop");

        let profiler = Profiler::start("\tProcess Input");
        let input_buffer = pancurses_renderer_system.get_input();

        for input in &input_buffer {
            if let pancurses::Input::Character('\u{1b}') = input {
                return Ok(());
            }
        }

        pancurses_input_system.set_input_buffer(&input_buffer);

        profiler.finish();

        pancurses_input_system.run(&mut ecs)?;

        for (system_name, system) in &mut systems {
            let profiler = Profiler::start(&format!("\tRun {}", system_name));
            system.run(&mut ecs)?;
            profiler.finish();
        }

        let profiler = Profiler::start("\tRun Pancurses Renderer System");
        pancurses_renderer_system.run(&mut ecs)?;
        profiler.finish();

        main_loop_profiler.finish();
    }
}

fn register_assemblages(ecs: &mut impl ECS) -> HashMap<EntityAssemblage, AssemblageID> {
    vec![
        (
            EntityAssemblage::Player,
            ecs.build_assemblage(
                "Player Entity",
                "Controllable ASCII character with position and velocity",
            )
            .component(PancursesControlComponent::new(0, ControlData::String))
            .component(CharComponent::new('@'))
            .component(PancursesInputBufferComponent::default())
            .component(PositionComponent::new(IVector2(1, 1)))
            .component(VelocityComponent::new(IVector2(1, 1)))
            .finish(),
        ),
        (
            EntityAssemblage::StringControl,
            ecs.build_assemblage("String Entity", "ASCII string control")
                .component(PancursesControlComponent::new(0, ControlData::String))
                .component(StringComponent::default())
                .component(PositionComponent::default())
                .finish(),
        ),
        (
            EntityAssemblage::RectControl,
            ecs.build_assemblage("Rect Entity", "ASCII Rectangle control")
                .component(PancursesControlComponent::new(
                    0,
                    ControlData::Rect { filled: true },
                ))
                .component(PositionComponent::default())
                .component(SizeComponent::default())
                .component(CharComponent::default())
                .component(PancursesColorPairComponent::new(PancursesColorPair::new(
                    PancursesColor::new(0, 0, 0),
                    PancursesColor::new(753, 753, 753),
                )))
                .finish(),
        ),
        (
            EntityAssemblage::BorderControl,
            ecs.build_assemblage("Border Entity", "ASCII Border control")
                .component(PancursesControlComponent::new(
                    0,
                    ControlData::Rect { filled: false },
                ))
                .component(PositionComponent::default())
                .component(SizeComponent::default())
                .component(CharComponent::default())
                .component(PancursesColorPairComponent::new(PancursesColorPair::new(
                    PancursesColor::new(0, 0, 0),
                    PancursesColor::new(753, 753, 753),
                )))
                .finish(),
        ),
    ]
    .into_iter()
    .collect()
}
