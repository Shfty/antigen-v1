mod components;
mod pancurses_color;
mod systems;

use std::{collections::HashMap, time::Duration};

use antigen::{
    components::DebugEntityListComponent,
    components::DebugExcludeComponent,
    components::StringListComponent,
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
    control_component::ControlComponent, fill_component::FillComponent,
    list_component::ListComponent, pancurses_color_pair_component::PancursesColorPairComponent,
    pancurses_input_buffer_component::PancursesInputBufferComponent,
    pancurses_input_axis_component::PancursesInputAxisComponent,
    pancurses_window_component::PancursesWindowComponent,
};
use pancurses_color::{PancursesColor, PancursesColorPair};
use systems::{
    InputVelocitySystem, ListSystem, PancursesInputSystem, PancursesInputAxisSystem,
    PancursesRendererSystem, PancursesWindowSystem,
};

// TODO: Update an 'InspectedEntityComponent' based on the selected entity list index
// TODO: Use the InspectedEntityComponent to drive the entity component list
// TODO: UI Anchor System
// TODO: Mouse input
// TODO: Scene tree mode for entity list window
// TODO: Component list mode for entity component window
// TODO: Profiler singleton (system?)
// TODO: Profiler menu

#[derive(Eq, PartialEq, Hash)]
enum EntityAssemblage {
    Player = 0,
    StringControl = 1,
    RectControl = 2,
    BorderControl = 3,
    DebugExclude = 4,
}

// Main Logic
fn main() {
    if let Err(err) = main_internal() {
        eprintln!("{}", err);
        std::process::exit(1);
    }
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
    window_id: i64,
    position: IVector2,
    size: IVector2,
    parent_window_entity_id: Option<EntityID>,
) -> Result<EntityID, String> {
    let entity_id = db.create_entity(label);
    db.add_component_to_entity(entity_id, PancursesWindowComponent::new(window_id))?;
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

fn create_game_window(
    db: &mut impl EntityComponentDatabase,
    assemblages: &HashMap<EntityAssemblage, Assemblage>,
    parent_window_entity: EntityID,
) -> Result<EntityID, String> {
    // Create Game Window
    let game_window_entity = create_window_entity(
        db,
        "Game Window",
        2,
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
) -> Result<EntityID, String> {
    let entity_list_window_entity = create_window_entity(
        db,
        "Entity List Window",
        1,
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

    let debug_exclude_assemblage = Assemblage::build(
        db,
        "Debug Exclude",
        "Assemblage to exclude entities from debug visualization",
    )
    .add_component(DebugExcludeComponent)
    .finish();

    let debug_list_entity = db.create_entity("Entity List");
    db.add_component_to_entity(
        debug_list_entity,
        ListComponent::new(Some(debug_list_entity), Some(debug_exclude_assemblage)),
    )?;
    db.add_component_to_entity(debug_list_entity, PancursesInputBufferComponent::default())?;
    db.add_component_to_entity(
        debug_list_entity,
        PancursesInputAxisComponent::new(
            pancurses::Input::KeyPPage,
            pancurses::Input::KeyNPage,
        ),
    )?;
    db.add_component_to_entity(debug_list_entity, IntRangeComponent::default())?;
    db.add_component_to_entity(debug_list_entity, PositionComponent::new(IVector2(1, 3)))?;
    db.add_component_to_entity(
        debug_list_entity,
        ParentEntityComponent::new(entity_list_window_entity),
    )?;
    db.add_component_to_entity::<DebugEntityListComponent>(
        debug_list_entity,
        DebugEntityListComponent,
    )?;
    db.add_component_to_entity::<StringListComponent>(
        debug_list_entity,
        StringListComponent::default(),
    )?;

    Ok(entity_list_window_entity)
}

fn create_component_list_window(
    db: &mut impl EntityComponentDatabase,
    assemblages: &HashMap<EntityAssemblage, Assemblage>,
    parent_window_entity: EntityID,
) -> Result<EntityID, String> {
    let component_list_window_entity = create_window_entity(
        db,
        "Component List Window",
        1,
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

    Ok(component_list_window_entity)
}

fn main_internal() -> Result<(), String> {
    let mut db = SingleThreadedDatabase::new();

    let assemblages = setup_assemblages(&mut db);

    // Create Main Window
    let main_window_entity = create_window_entity(
        &mut db,
        "Main Window",
        0,
        IVector2::default(),
        IVector2(128, 32),
        None,
    )?;

    create_game_window(&mut db, &assemblages, main_window_entity)?;
    create_entity_list_window(&mut db, &assemblages, main_window_entity)?;
    create_component_list_window(&mut db, &assemblages, main_window_entity)?;

    // Create systems
    let mut pancurses_window_system = PancursesWindowSystem::new();
    let mut pancurses_input_system = PancursesInputSystem::new(1);
    let mut pancurses_prev_next_input_system = PancursesInputAxisSystem::new();
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
    system_runner.register_system("ECS Debug", &mut ecs_debug_system);
    system_runner.register_system("List", &mut list_system);
    system_runner.register_system("Input Velocity", &mut input_velocity_system);
    system_runner.register_system("Position Integrator", &mut position_integrator_system);
    system_runner.register_system("Global Position", &mut global_position_system);
    system_runner.register_system("Pancurses Renderer", &mut pancurses_renderer_system);

    // Main loop
    loop {
        let main_loop_profiler = Profiler::start("Main Loop");

        if let Ok(SystemEvent::Quit) = system_runner.run() {
            return Ok(());
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

    assemblages
}
