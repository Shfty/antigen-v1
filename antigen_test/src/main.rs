mod components;
mod pancurses_color;
mod systems;

use std::{collections::HashMap, ops::Range, time::Duration};

use antigen::{
    components::ComponentDebugComponent,
    components::EntityDebugComponent,
    components::{
        AnchorsComponent, CharComponent, ComponentInspectorComponent,
        DebugComponentDataListComponent, DebugComponentListComponent, DebugEntityListComponent,
        DebugExcludeComponent, EntityInspectorComponent, GlobalPositionComponent,
        IntRangeComponent, MarginsComponent, ParentEntityComponent, PositionComponent,
        SizeComponent, StringComponent, StringListComponent, VelocityComponent, WindowComponent,
        ZIndexComponent,
    },
    entity_component_system::entity_component_database::ComponentStorage,
    entity_component_system::entity_component_database::EntityComponentDatabase,
    entity_component_system::entity_component_database::EntityComponentDirectory,
    entity_component_system::entity_component_database::HeapComponentStorage,
    entity_component_system::entity_component_database::SingleThreadedDirectory,
    entity_component_system::system_runner::SingleThreadedSystemRunner,
    entity_component_system::SystemRunner,
    entity_component_system::{Assemblage, EntityID, SystemError},
    primitive_types::IVector2,
    profiler::Profiler,
    systems::AnchorsMarginsSystem,
    systems::ChildEntitiesSystem,
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

// TODO: Minimize usage of dyn in favor of generics for trait objects
// TODO: Remove debug system dependency on get_component_data_dyn, remove *_dyn_* functions from trait
// TODO: Create EntityComponentSystem struct to compose EntityComponentDatabase + SystemRunner
// TODO: Create storage class for systems

// TODO: Event synthesis for list hover / click
//       Global 'event queue' entity w/component containing array of boxed event traits (heap allocating on every event seems like a bad idea)
//       Emitter systems push events into the array
//       Receiver systems iterate through the array, respond to, and optionally consume events
//       How to avoid memory leaks from un-consumed events? Clear at beginning of frame and assume emitters will always run before receivers?

// TODO: Click to set inspected entity / component

// TODO: System list window

// TODO: Scene tree window
// TODO: Profiler singleton (system?)
// TODO: Profiler menu

// TODO: Clipping rects for UI controls

// TODO: Register entity fetches and associated logic at System registration time to allow for runtime lookup optimization
//       Ex:
//          db
//            .entity_query()
//            .component::<PositionComponent>()
//            .and(
//                  db
//                 .entity_query()
//                 .component::<SizeComponent>()
//                 .or(
//                     db
//                     .entity_query()
//                     .component::<AnchorsComponent>()
//                 )
//             )
//             .then(|entity_id| {
//                  do_something_with_position_component();
//                  maybe_do_something_with_size_component()
//                  maybe_do_something_with_anchors_component()
//             })
//             .register()
//
//

#[derive(Eq, PartialEq, Hash)]
enum EntityAssemblage {
    Player = 0,
    StringControl = 1,
    RectControl = 2,
    BorderControl = 3,
    DestructionTest = 4,
}

fn create_string_control<S, D>(
    db: &mut EntityComponentDatabase<S, D>,
    string_assemblage: &mut Assemblage<S, D>,
    debug_label: Option<&str>,
    text: &str,
    (x, y): (i64, i64),
) -> Result<EntityID, String>
where
    S: ComponentStorage,
    D: EntityComponentDirectory,
{
    let entity_id = string_assemblage.create_and_assemble_entity(db, debug_label)?;

    db.get_entity_component_mut::<StringComponent>(entity_id)?
        .set_data(text.into());

    db.get_entity_component_mut::<PositionComponent>(entity_id)?
        .set_position(IVector2(x, y));

    Ok(entity_id)
}

fn create_window_entity<S, D>(
    db: &mut EntityComponentDatabase<S, D>,
    debug_label: Option<&str>,
    position: IVector2,
    size: IVector2,
    parent_window_entity_id: Option<EntityID>,
) -> Result<EntityID, String>
where
    S: ComponentStorage,
    D: EntityComponentDirectory,
{
    let entity_id = db.create_entity(debug_label)?;
    db.insert_entity_component(entity_id, WindowComponent)?;
    db.insert_entity_component(entity_id, PancursesWindowComponent::default())?;
    db.insert_entity_component(entity_id, PositionComponent::new(position))?;
    db.insert_entity_component(entity_id, SizeComponent::new(size))?;
    if let Some(parent_window_entity_id) = parent_window_entity_id {
        db.insert_entity_component(
            entity_id,
            ParentEntityComponent::new(parent_window_entity_id),
        )?;
    }
    Ok(entity_id)
}

fn setup_assemblages<S, D>() -> Result<HashMap<EntityAssemblage, Assemblage<S, D>>, String>
where
    S: ComponentStorage,
    D: EntityComponentDirectory,
{
    let mut assemblages: HashMap<EntityAssemblage, Assemblage<S, D>> = HashMap::new();

    assemblages.insert(
        EntityAssemblage::Player,
        Assemblage::build(
            "Player Entity",
            "Controllable ASCII character with position and velocity",
        )
        .add_component(ControlComponent)?
        .add_component(PancursesColorPairComponent::new(PancursesColorPair::new(
            PancursesColor::new(1000, 600, 1000),
            PancursesColor::new(1000, 1000, 1000),
        )))?
        .add_component(CharComponent::new('@'))?
        .add_component(PancursesInputBufferComponent::default())?
        .add_component(PositionComponent::new(IVector2(1, 1)))?
        .add_component(VelocityComponent::new(IVector2(1, 1)))?
        .finish(),
    );

    assemblages.insert(
        EntityAssemblage::StringControl,
        Assemblage::build("String Entity", "ASCII string control")
            .add_component(ControlComponent)?
            .add_component(StringComponent::default())?
            .add_component(PositionComponent::default())?
            .finish(),
    );

    assemblages.insert(
        EntityAssemblage::RectControl,
        Assemblage::build("Rect Entity", "ASCII Rectangle control")
            .add_component(ControlComponent)?
            .add_component(PositionComponent::default())?
            .add_component(SizeComponent::default())?
            .add_component(CharComponent::default())?
            .add_component(FillComponent)?
            .add_component(PancursesColorPairComponent::new(PancursesColorPair::new(
                PancursesColor::new(0, 0, 0),
                PancursesColor::new(753, 753, 753),
            )))?
            .finish(),
    );

    assemblages.insert(
        EntityAssemblage::BorderControl,
        Assemblage::build("Border Entity", "ASCII Border control")
            .add_component(ControlComponent)?
            .add_component(PositionComponent::default())?
            .add_component(SizeComponent::default())?
            .add_component(CharComponent::default())?
            .add_component(PancursesColorPairComponent::new(PancursesColorPair::new(
                PancursesColor::new(0, 0, 0),
                PancursesColor::new(753, 753, 753),
            )))?
            .finish(),
    );

    assemblages.insert(
        EntityAssemblage::DestructionTest,
        Assemblage::build(
            "Destruction Test",
            "Assemblage for destroying entities when space is pressed",
        )
        .add_component(DestructionTestInputComponent::new(' '))?
        .add_component(PancursesInputBufferComponent::default())?
        .finish(),
    );

    Ok(assemblages)
}

fn create_game_window<S, D>(
    db: &mut EntityComponentDatabase<S, D>,
    assemblages: &mut HashMap<EntityAssemblage, Assemblage<S, D>>,
    parent_window_entity: EntityID,
) -> Result<EntityID, String>
where
    S: ComponentStorage,
    D: EntityComponentDirectory,
{
    // Create Game Window
    let game_window_entity = db.create_entity(Some("Game"))?;
    db.insert_entity_component(game_window_entity, PositionComponent::default())?;
    db.insert_entity_component(game_window_entity, SizeComponent::default())?;
    db.insert_entity_component(
        game_window_entity,
        ParentEntityComponent::new(parent_window_entity),
    )?;

    db.insert_entity_component(
        game_window_entity,
        AnchorsComponent::new(0.0..0.5, 0.0..1.0),
    )?;

    // Create Test Player
    let test_player_entity = assemblages
        .get_mut(&EntityAssemblage::Player)
        .unwrap()
        .create_and_assemble_entity(db, Some("Test Player"))?;
    db.insert_entity_component(
        test_player_entity,
        ParentEntityComponent::new(game_window_entity),
    )?;
    db.insert_entity_component(test_player_entity, GlobalPositionComponent::default())?;

    // Create Test String
    let test_string_entity = assemblages
        .get_mut(&EntityAssemblage::StringControl)
        .unwrap()
        .create_and_assemble_entity(db, Some("Test String Control"))?;
    db.get_entity_component_mut::<PositionComponent>(test_string_entity)?
        .set_position(IVector2(1, 1));
    db.get_entity_component_mut::<StringComponent>(test_string_entity)?
        .set_data("Testing One Two Three".into());
    db.insert_entity_component(
        test_string_entity,
        ParentEntityComponent::new(test_player_entity),
    )?;
    db.insert_entity_component(test_string_entity, GlobalPositionComponent::default())?;

    // Create Test Rect
    let test_rect_entity = assemblages
        .get_mut(&EntityAssemblage::RectControl)
        .unwrap()
        .create_and_assemble_entity(db, Some("Test Rect Control"))?;

    db.get_entity_component_mut::<PositionComponent>(test_rect_entity)?
        .set_position(IVector2(1, 5));

    db.get_entity_component_mut::<SizeComponent>(test_rect_entity)?
        .set_size(IVector2(20, 5));
    db.insert_entity_component(
        test_rect_entity,
        ParentEntityComponent::new(test_player_entity),
    )?;
    db.insert_entity_component(test_rect_entity, GlobalPositionComponent::default())?;

    Ok(game_window_entity)
}

fn create_debug_window<S, D>(
    db: &mut EntityComponentDatabase<S, D>,
    assemblages: &mut HashMap<EntityAssemblage, Assemblage<S, D>>,
    parent_window_entity: EntityID,
    list_index_entity: Option<EntityID>,
    window_name: &str,
    anchor_horizontal: Range<f32>,
    anchor_vertical: Range<f32>,
) -> Result<EntityID, String>
where
    S: ComponentStorage,
    D: EntityComponentDirectory,
{
    let entity_list_window_entity = assemblages
        .get_mut(&EntityAssemblage::RectControl)
        .unwrap()
        .create_and_assemble_entity(db, Some(&format!("{} Window", window_name)))?;
    db.get_entity_component_mut::<PancursesColorPairComponent>(entity_list_window_entity)?
        .set_data(PancursesColorPair::new(
            PancursesColor::new(0, 0, 0),
            PancursesColor::new(0, 0, 0),
        ));
    db.insert_entity_component(entity_list_window_entity, PositionComponent::default())?;
    db.insert_entity_component(entity_list_window_entity, ZIndexComponent::new(1))?;
    db.insert_entity_component(entity_list_window_entity, SizeComponent::default())?;
    db.insert_entity_component(
        entity_list_window_entity,
        ParentEntityComponent::new(parent_window_entity),
    )?;
    db.insert_entity_component(
        entity_list_window_entity,
        AnchorsComponent::new(anchor_horizontal, anchor_vertical),
    )?;

    let entity_list_border_entity = assemblages
        .get_mut(&EntityAssemblage::BorderControl)
        .unwrap()
        .create_and_assemble_entity(db, Some(&format!("{} Border", window_name)))?;
    db.insert_entity_component(
        entity_list_border_entity,
        ParentEntityComponent::new(entity_list_window_entity),
    )?;
    db.insert_entity_component(
        entity_list_border_entity,
        AnchorsComponent::new(0.0..1.0, 0.0..1.0),
    )?;

    // Create Debug Window Title
    let entity_list_title_entity = create_string_control(
        db,
        assemblages
            .get_mut(&EntityAssemblage::StringControl)
            .unwrap(),
        Some(&format!("{} Title", window_name)),
        &format!("{}\n========", window_name),
        (2, 1),
    )?;
    db.insert_entity_component(
        entity_list_title_entity,
        ParentEntityComponent::new(entity_list_border_entity),
    )?;
    db.insert_entity_component(entity_list_title_entity, GlobalPositionComponent::default())?;

    // Create Entity List
    let entity_list_entity = db.create_entity(Some(window_name))?;
    db.insert_entity_component(
        entity_list_entity,
        ListComponent::new(Some(entity_list_entity), list_index_entity),
    )?;
    db.insert_entity_component(entity_list_entity, PositionComponent::default())?;
    db.insert_entity_component(entity_list_entity, SizeComponent::default())?;
    db.insert_entity_component(
        entity_list_entity,
        ParentEntityComponent::new(entity_list_border_entity),
    )?;
    db.insert_entity_component(
        entity_list_entity,
        AnchorsComponent::new(0.0..1.0, 0.0..1.0),
    )?;
    db.insert_entity_component(entity_list_entity, MarginsComponent::new(2, 2, 3, 1))?;
    db.insert_entity_component(entity_list_entity, StringListComponent::default())?;
    db.insert_entity_component(entity_list_entity, LocalMousePositionComponent::default())?;
    db.insert_entity_component(entity_list_entity, DebugExcludeComponent)?;

    Ok(entity_list_entity)
}

fn create_entity_list_window<S, D>(
    db: &mut EntityComponentDatabase<S, D>,
    assemblages: &mut HashMap<EntityAssemblage, Assemblage<S, D>>,
    parent_window_entity: EntityID,
    entity_inspector_entity: EntityID,
) -> Result<EntityID, String>
where
    S: ComponentStorage,
    D: EntityComponentDirectory,
{
    let entity_list_entity = create_debug_window(
        db,
        assemblages,
        parent_window_entity,
        Some(entity_inspector_entity),
        "Entities",
        0.25..0.5,
        0.0..0.5,
    )?;
    db.insert_entity_component(entity_list_entity, DebugEntityListComponent)?;
    Ok(entity_list_entity)
}

fn create_component_list_window<S, D>(
    db: &mut EntityComponentDatabase<S, D>,
    assemblages: &mut HashMap<EntityAssemblage, Assemblage<S, D>>,
    parent_window_entity: EntityID,
    component_inspector_entity: EntityID,
) -> Result<EntityID, String>
where
    S: ComponentStorage,
    D: EntityComponentDirectory,
{
    let component_list_entity = create_debug_window(
        db,
        assemblages,
        parent_window_entity,
        Some(component_inspector_entity),
        "Components",
        0.5..0.75,
        0.0..1.0,
    )?;
    db.insert_entity_component(component_list_entity, DebugComponentListComponent)?;
    db.insert_entity_component(component_list_entity, DebugExcludeComponent)?;

    Ok(component_list_entity)
}

fn create_component_data_list_window<S, D>(
    db: &mut EntityComponentDatabase<S, D>,
    assemblages: &mut HashMap<EntityAssemblage, Assemblage<S, D>>,
    parent_window_entity: EntityID,
) -> Result<EntityID, String>
where
    S: ComponentStorage,
    D: EntityComponentDirectory,
{
    let component_list_entity = create_debug_window(
        db,
        assemblages,
        parent_window_entity,
        None,
        "Component Data",
        0.75..1.0,
        0.0..1.0,
    )?;
    db.insert_entity_component(component_list_entity, DebugComponentDataListComponent)?;
    Ok(component_list_entity)
}

fn setup_debug_system<S, D>(
    db: &mut EntityComponentDatabase<S, D>,
) -> Result<ECSDebugSystem<D>, String>
where
    S: ComponentStorage,
    D: EntityComponentDirectory,
{
    let ecs_debug_system = ECSDebugSystem::new(db);

    let entity_debug_entity = db.create_entity(None)?;

    db.insert_entity_component(entity_debug_entity, EntityDebugComponent::default())?
        .register_entity(entity_debug_entity, "Entity Debug".into());

    db.insert_entity_component(entity_debug_entity, DebugExcludeComponent)?;

    let component_debug_entity = db.create_entity("Component Debug".into())?;

    db.insert_entity_component(component_debug_entity, ComponentDebugComponent::default())?;
    db.insert_entity_component(component_debug_entity, DebugExcludeComponent)?;

    Ok(ecs_debug_system)
}

fn create_entities<S, D>(
    db: &mut EntityComponentDatabase<S, D>,
    assemblages: &mut HashMap<EntityAssemblage, Assemblage<S, D>>,
) -> Result<(), String>
where
    S: ComponentStorage,
    D: EntityComponentDirectory,
{
    // Create Main Window
    let main_window_entity = create_window_entity(
        db,
        Some("Main Window"),
        IVector2::default(),
        IVector2(256, 64),
        None,
    )?;

    let entity_inspector_entity = db.create_entity(Some("Entity Inspector"))?;
    db.insert_entity_component(entity_inspector_entity, EntityInspectorComponent)?;
    db.insert_entity_component(entity_inspector_entity, IntRangeComponent::default())?;

    let component_inspector_entity = db.create_entity(Some("Component Inspector"))?;
    db.insert_entity_component(component_inspector_entity, ComponentInspectorComponent)?;
    db.insert_entity_component(component_inspector_entity, IntRangeComponent::default())?;

    create_game_window(db, assemblages, main_window_entity)?;
    create_entity_list_window(db, assemblages, main_window_entity, entity_inspector_entity)?;
    create_component_list_window(
        db,
        assemblages,
        main_window_entity,
        component_inspector_entity,
    )?;
    create_component_data_list_window(db, assemblages, main_window_entity)?;

    Ok(())
}

fn register_systems<S, D>(
    system_runner: &mut SingleThreadedSystemRunner<S, D>,
    db: &mut EntityComponentDatabase<S, D>,
    ecs_debug_system: ECSDebugSystem<D>,
) where
    S: ComponentStorage,
    D: EntityComponentDirectory + 'static,
{
    system_runner.register_system("Pancurses Window", PancursesWindowSystem::new(db));
    system_runner.register_system("Pancurses Input", PancursesInputSystem::new(1));
    system_runner.register_system("Pancurses Input Axis", PancursesInputAxisSystem::new());
    system_runner.register_system("Destruction Test Input", DestructionTestInputSystem::new());
    system_runner.register_system("Local Mouse Position", LocalMousePositionSystem::new());
    system_runner.register_system("ECS Debug", ecs_debug_system);
    system_runner.register_system("List", ListSystem::new());
    system_runner.register_system("Input Velocity", InputVelocitySystem::new());
    system_runner.register_system("Position Integrator", PositionIntegratorSystem::new());
    system_runner.register_system("Anchors Margins", AnchorsMarginsSystem::new());
    system_runner.register_system("Global Position", GlobalPositionSystem::new());
    system_runner.register_system("Child Entities", ChildEntitiesSystem::new());
    system_runner.register_system("Pancurses Renderer", PancursesRendererSystem::new());
}

fn main_internal() -> Result<(), SystemError> {
    // Create entity-component database
    let mut db =
        EntityComponentDatabase::new(HeapComponentStorage::new(), SingleThreadedDirectory::new());

    // Debug system has to be initialized before any entities or components are registered
    let ecs_debug_system = setup_debug_system(&mut db)?;

    // Create entities
    let mut assemblages = setup_assemblages()?;
    create_entities(&mut db, &mut assemblages)?;

    // Create systems
    let mut system_runner =
        SingleThreadedSystemRunner::<HeapComponentStorage, SingleThreadedDirectory>::new();
    register_systems(&mut system_runner, &mut db, ecs_debug_system);

    // Main loop
    let frame_time_target = Duration::from_secs_f32(1.0 / 60.0);
    loop {
        let main_loop_profiler = Profiler::start("Main Loop");
        system_runner.run(&mut db)?;
        let delta = main_loop_profiler.finish();

        // Sleep if framerate target is exceeded - prevents deadlock when pancurses stops being able to poll input after window close
        if delta < frame_time_target {
            let delta = frame_time_target - delta;
            std::thread::sleep(delta);
        }
    }
}

fn main() {
    if let Err(err) = main_internal() {
        match err {
            SystemError::Err(err) => {
                eprintln!("{}", err);
                std::process::exit(1)
            }
            SystemError::Quit => std::process::exit(0),
        }
    }
}
