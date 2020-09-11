mod components;
mod pancurses_color;
mod systems;

use std::{collections::HashMap, time::Duration};

use antigen::{
    components::DebugEntityComponentListComponent,
    components::DebugEntityListComponent,
    components::DebugExcludeComponent,
    components::EntityInspectorComponent,
    components::StringListComponent,
    components::WindowComponent,
    components::{
        CharComponent, GlobalPositionComponent, IntRangeComponent, ParentEntityComponent,
        PositionComponent, SizeComponent, StringComponent, VelocityComponent,
    },
    ecs::entity_component_database::SingleThreadedDatabase,
    ecs::system_runner::SingleThreadedSystemRunner,
    ecs::SystemRunner,
    ecs::{Assemblage, EntityComponentDatabase, EntityID, SystemEvent},
    primitive_types::IVector2,
    profiler::Profiler,
    systems::{ECSDebugSystem, GlobalPositionSystem, PositionIntegratorSystem},
};

use components::{
    control_component::ControlComponent,
    destruction_test_input_component::DestructionTestInputComponent, fill_component::FillComponent,
    list_component::ListComponent, local_mouse_position_component::LocalMousePositionComponent,
    pancurses_color_pair_component::PancursesColorPairComponent,
    pancurses_input_buffer_component::PancursesInputBufferComponent,
    pancurses_window_component::PancursesWindowComponent,
};
use pancurses_color::{PancursesColor, PancursesColorPair};
use systems::{
    DestructionTestInputSystem, InputVelocitySystem, ListSystem, LocalMousePositionSystem,
    PancursesInputAxisSystem, PancursesInputSystem, PancursesRendererSystem, PancursesWindowSystem,
};

// TODO: UI Anchor System
// TODO: Couple item index to substrings in list system, use for highlighting and mouse hit tests
// TODO: Reimplement window teardown
// TODO: Implement window resizing
// TODO: Refactor entity component window into component window, filter based on inspected entity
// TODO: Component data window, filter based on inspected component
// TODO: Event synthesis for list hover / click
//       Global 'event queue' entity w/component containing array of boxed event traits
//       Systems iterate through component's array, respond to and optionally(? - they'll need to be removed from the array eventually) consume events
// TODO: Click to set inspected entity / component
// TODO: Refactor SystemEvent into Error enum with String and Quit variants, return () for system success
// TODO: System list window
// TODO: Scene tree window
// TODO: Profiler singleton (system?)
// TODO: Profiler menu

#[derive(Eq, PartialEq, Hash)]
enum EntityAssemblage {
    Player = 0,
    StringControl = 1,
    RectControl = 2,
    BorderControl = 3,
    DebugExclude = 4,
    DestructionTest = 5,
}

fn create_string_control(
    db: &mut impl EntityComponentDatabase,
    string_assemblage: &Assemblage,
    label: &str,
    text: &str,
    (x, y): (i64, i64),
) -> Result<EntityID, String> {
    let entity_id = string_assemblage.create_and_assemble_entity(db, label)?;

    let string_component = db.get_entity_component_mut::<StringComponent>(entity_id)?;
    string_component.data = text.into();

    let position_component = db.get_entity_component_mut::<PositionComponent>(entity_id)?;
    let IVector2(pos_x, pos_y) = &mut position_component.data;
    *pos_x = x;
    *pos_y = y;

    Ok(entity_id)
}

fn create_window_entity(
    db: &mut impl EntityComponentDatabase,
    label: &str,
    position: IVector2,
    size: IVector2,
    parent_window_entity_id: Option<EntityID>,
) -> Result<EntityID, String> {
    let entity_id = db.create_entity(label);
    db.add_component_to_entity(entity_id, WindowComponent)?;
    db.add_component_to_entity(entity_id, PancursesWindowComponent::default())?;
    db.add_component_to_entity(entity_id, PositionComponent::new(position))?;
    db.add_component_to_entity(entity_id, SizeComponent::new(size))?;
    if let Some(parent_window_entity_id) = parent_window_entity_id {
        db.add_component_to_entity(
            entity_id,
            ParentEntityComponent::new(parent_window_entity_id),
        )?;
    }
    Ok(entity_id)
}

fn setup_assemblages(
    db: &mut impl EntityComponentDatabase,
) -> HashMap<EntityAssemblage, Assemblage> {
    let mut assemblages: HashMap<EntityAssemblage, Assemblage> = HashMap::new();

    assemblages.insert(
        EntityAssemblage::Player,
        Assemblage::build(
            db,
            "Player Entity",
            "Controllable ASCII character with position and velocity",
        )
        .add_component(ControlComponent)
        .add_component(PancursesColorPairComponent::new(PancursesColorPair::new(
            PancursesColor::new(1000, 600, 1000),
            PancursesColor::new(1000, 1000, 1000),
        )))
        .add_component(CharComponent::new('@'))
        .add_component(PancursesInputBufferComponent::default())
        .add_component(PositionComponent::new(IVector2(1, 1)))
        .add_component(VelocityComponent::new(IVector2(1, 1)))
        .finish(),
    );

    assemblages.insert(
        EntityAssemblage::StringControl,
        Assemblage::build(db, "String Entity", "ASCII string control")
            .add_component(ControlComponent)
            .add_component(StringComponent::default())
            .add_component(PositionComponent::default())
            .finish(),
    );

    assemblages.insert(
        EntityAssemblage::RectControl,
        Assemblage::build(db, "Rect Entity", "ASCII Rectangle control")
            .add_component(ControlComponent)
            .add_component(PositionComponent::default())
            .add_component(SizeComponent::default())
            .add_component(CharComponent::default())
            .add_component(FillComponent)
            .add_component(PancursesColorPairComponent::new(PancursesColorPair::new(
                PancursesColor::new(0, 0, 0),
                PancursesColor::new(753, 753, 753),
            )))
            .finish(),
    );

    assemblages.insert(
        EntityAssemblage::BorderControl,
        Assemblage::build(db, "Border Entity", "ASCII Border control")
            .add_component(ControlComponent)
            .add_component(PositionComponent::default())
            .add_component(SizeComponent::default())
            .add_component(CharComponent::default())
            .add_component(PancursesColorPairComponent::new(PancursesColorPair::new(
                PancursesColor::new(0, 0, 0),
                PancursesColor::new(753, 753, 753),
            )))
            .finish(),
    );

    assemblages.insert(
        EntityAssemblage::DebugExclude,
        Assemblage::build(
            db,
            "Debug Exclude",
            "Assemblage to exclude entities from debug visualization",
        )
        .add_component(DebugExcludeComponent)
        .finish(),
    );

    assemblages.insert(
        EntityAssemblage::DestructionTest,
        Assemblage::build(
            db,
            "Destruction Test",
            "Assemblage for destroying entities when space is pressed",
        )
        .add_component(DestructionTestInputComponent::new(' '))
        .add_component(PancursesInputBufferComponent::default())
        .finish(),
    );

    assemblages
}

fn create_game_window(
    db: &mut impl EntityComponentDatabase,
    assemblages: &HashMap<EntityAssemblage, Assemblage>,
    parent_window_entity: EntityID,
) -> Result<EntityID, String> {
    // Create Game Window
    let game_window_entity = create_window_entity(
        db,
        "Game Window",
        IVector2(32, 0),
        IVector2(64, 32),
        Some(parent_window_entity),
    )?;

    // Create Test Player
    let test_player_entity =
        assemblages[&EntityAssemblage::Player].create_and_assemble_entity(db, "Test Player")?;
    db.add_component_to_entity(
        test_player_entity,
        ParentEntityComponent::new(game_window_entity),
    )?;

    // Create Test String
    let test_string_entity = assemblages[&EntityAssemblage::StringControl]
        .create_and_assemble_entity(db, "Test String Control")?;
    if let Ok(position_component) =
        db.get_entity_component_mut::<PositionComponent>(test_string_entity)
    {
        position_component.data = IVector2(1, 1);
    }
    if let Ok(string_component) = db.get_entity_component_mut::<StringComponent>(test_string_entity)
    {
        string_component.data = "Testing One Two Three".into();
    }
    db.add_component_to_entity(
        test_string_entity,
        ParentEntityComponent::new(test_player_entity),
    )?;
    db.add_component_to_entity(test_string_entity, GlobalPositionComponent::default())?;

    // Create Test Rect
    let test_rect_entity = assemblages[&EntityAssemblage::RectControl]
        .create_and_assemble_entity(db, "Test Rect Control")?;
    if let Ok(position_component) =
        db.get_entity_component_mut::<PositionComponent>(test_rect_entity)
    {
        position_component.data = IVector2(1, 5);
    }
    if let Ok(size_component) = db.get_entity_component_mut::<SizeComponent>(test_rect_entity) {
        size_component.data = IVector2(20, 5);
    }
    db.add_component_to_entity(
        test_rect_entity,
        ParentEntityComponent::new(test_player_entity),
    )?;
    db.add_component_to_entity(test_rect_entity, GlobalPositionComponent::default())?;

    Ok(game_window_entity)
}

fn create_entity_list_window(
    db: &mut impl EntityComponentDatabase,
    assemblages: &HashMap<EntityAssemblage, Assemblage>,
    parent_window_entity: EntityID,
    entity_inspector_entity: EntityID,
) -> Result<EntityID, String> {
    let entity_list_window_entity = create_window_entity(
        db,
        "Entity List Window",
        IVector2(0, 0),
        IVector2(32, 32),
        Some(parent_window_entity),
    )?;

    let entity_list_border_entity = assemblages[&EntityAssemblage::BorderControl]
        .create_and_assemble_entity(db, "Entity List Border")?;
    if let Ok(size_component) =
        db.get_entity_component_mut::<SizeComponent>(entity_list_border_entity)
    {
        size_component.data = IVector2(32, 32);
    }
    db.add_component_to_entity(
        entity_list_border_entity,
        ParentEntityComponent::new(entity_list_window_entity),
    )?;

    // Create Debug Window Title
    let entity_list_title_entity = create_string_control(
        db,
        &assemblages[&EntityAssemblage::StringControl],
        "Entity List Title",
        "Entities\n========",
        (1, 1),
    )?;
    db.add_component_to_entity(
        entity_list_title_entity,
        ParentEntityComponent::new(entity_list_window_entity),
    )?;

    // Create Entity List
    let entity_list_entity = db.create_entity("Entity List");
    db.add_component_to_entity(
        entity_list_entity,
        ListComponent::new(
            Some(entity_list_entity),
            Some(entity_inspector_entity),
            Some(assemblages[&EntityAssemblage::DebugExclude].clone()),
        ),
    )?;
    db.add_component_to_entity(entity_list_entity, PositionComponent::new(IVector2(1, 3)))?;
    db.add_component_to_entity(entity_list_entity, SizeComponent::new(IVector2(30, 30)))?;
    db.add_component_to_entity(
        entity_list_entity,
        ParentEntityComponent::new(entity_list_window_entity),
    )?;
    db.add_component_to_entity(entity_list_entity, DebugEntityListComponent)?;
    db.add_component_to_entity(entity_list_entity, StringListComponent::default())?;
    db.add_component_to_entity(entity_list_entity, LocalMousePositionComponent::default())?;

    Ok(entity_list_window_entity)
}

fn create_component_list_window(
    db: &mut impl EntityComponentDatabase,
    assemblages: &HashMap<EntityAssemblage, Assemblage>,
    parent_window_entity: EntityID,
    component_inspector_entity: EntityID,
) -> Result<EntityID, String> {
    let component_list_window_entity = create_window_entity(
        db,
        "Component List Window",
        IVector2(96, 0),
        IVector2(32, 32),
        Some(parent_window_entity),
    )?;

    let component_list_border_entity = assemblages[&EntityAssemblage::BorderControl]
        .create_and_assemble_entity(db, "Component List Border")?;
    if let Ok(size_component) =
        db.get_entity_component_mut::<SizeComponent>(component_list_border_entity)
    {
        size_component.data = IVector2(32, 32);
    }
    db.add_component_to_entity(
        component_list_border_entity,
        ParentEntityComponent::new(component_list_window_entity),
    )?;

    // Create Debug Window Title
    let entity_list_title_entity = create_string_control(
        db,
        &assemblages[&EntityAssemblage::StringControl],
        "Component List Title",
        "Entity Components\n=================",
        (1, 1),
    )?;
    db.add_component_to_entity(
        entity_list_title_entity,
        ParentEntityComponent::new(component_list_window_entity),
    )?;

    // Create Entity List
    let component_list_entity = db.create_entity("Component List");
    db.add_component_to_entity(
        component_list_entity,
        ListComponent::new(
            Some(component_list_entity),
            Some(component_inspector_entity),
            Some(assemblages[&EntityAssemblage::DebugExclude].clone()),
        ),
    )?;
    db.add_component_to_entity(
        component_list_entity,
        PositionComponent::new(IVector2(1, 3)),
    )?;
    db.add_component_to_entity(component_list_entity, SizeComponent::new(IVector2(30, 28)))?;
    db.add_component_to_entity(
        component_list_entity,
        ParentEntityComponent::new(component_list_window_entity),
    )?;
    db.add_component_to_entity(component_list_entity, DebugEntityComponentListComponent)?;
    db.add_component_to_entity(component_list_entity, StringListComponent::default())?;
    db.add_component_to_entity(
        component_list_entity,
        LocalMousePositionComponent::default(),
    )?;

    Ok(component_list_window_entity)
}

fn main_internal() -> Result<(), String> {
    let mut db = SingleThreadedDatabase::new();

    let assemblages = setup_assemblages(&mut db);

    // Create Main Window
    let main_window_entity = create_window_entity(
        &mut db,
        "Main Window",
        IVector2::default(),
        IVector2(128, 32),
        None,
    )?;

    assemblages[&EntityAssemblage::DestructionTest].assemble_entity(&mut db, main_window_entity)?;

    let entity_inspector_entity = db.create_entity("Entity Inspector");
    db.add_component_to_entity(entity_inspector_entity, EntityInspectorComponent)?;
    db.add_component_to_entity(entity_inspector_entity, IntRangeComponent::default())?;
    db.add_component_to_entity(
        entity_inspector_entity,
        PancursesInputBufferComponent::default(),
    )?;

    let component_inspector_entity = db.create_entity("Component Inspector");
    db.add_component_to_entity(component_inspector_entity, IntRangeComponent::default())?;

    create_game_window(&mut db, &assemblages, main_window_entity)?;
    create_entity_list_window(
        &mut db,
        &assemblages,
        main_window_entity,
        entity_inspector_entity,
    )?;
    create_component_list_window(
        &mut db,
        &assemblages,
        main_window_entity,
        component_inspector_entity,
    )?;

    // Create systems
    let mut pancurses_window_system = PancursesWindowSystem::new();
    let mut pancurses_input_system = PancursesInputSystem::new(1);
    let mut pancurses_prev_next_input_system = PancursesInputAxisSystem::new();
    let mut destruction_test_input_system = DestructionTestInputSystem::new();
    let mut local_mouse_position_system = LocalMousePositionSystem::new();
    let mut list_system = ListSystem::new();
    let mut input_velocity_system = InputVelocitySystem::new();
    let mut position_integrator_system = PositionIntegratorSystem::new();
    let mut global_position_system = GlobalPositionSystem::new();
    let mut ecs_debug_system = ECSDebugSystem::new();
    let mut pancurses_renderer_system = PancursesRendererSystem::new();

    let mut system_runner = SingleThreadedSystemRunner::<SingleThreadedDatabase>::new(&mut db);
    system_runner.register_system("Pancurses Window", &mut pancurses_window_system);
    system_runner.register_system("Pancurses Input", &mut pancurses_input_system);
    system_runner.register_system(
        "Pancurses Prev Next Input",
        &mut pancurses_prev_next_input_system,
    );
    system_runner.register_system("Destruction Test Input", &mut destruction_test_input_system);
    system_runner.register_system("Local Mouse Position", &mut local_mouse_position_system);
    system_runner.register_system("ECS Debug", &mut ecs_debug_system);
    system_runner.register_system("List", &mut list_system);
    system_runner.register_system("Input Velocity", &mut input_velocity_system);
    system_runner.register_system("Position Integrator", &mut position_integrator_system);
    system_runner.register_system("Global Position", &mut global_position_system);
    system_runner.register_system("Pancurses Renderer", &mut pancurses_renderer_system);

    // Main loop
    loop {
        let main_loop_profiler = Profiler::start("Main Loop");

        match system_runner.run() {
            Ok(SystemEvent::Quit) => return Ok(()),
            Err(err) => return Err(err),
            _ => (),
        }

        // Sleep if framerate target is exceeded - prevents deadlock when pancurses stops being able to poll input after window close
        let delta = main_loop_profiler.finish();
        let target = Duration::from_secs_f32(1.0 / 60.0);
        if delta < target {
            let delta = target - delta;
            std::thread::sleep(delta);
        }
    }
}

fn main() {
    if let Err(err) = main_internal() {
        eprintln!("{}", err);
        std::process::exit(1);
    }
}
