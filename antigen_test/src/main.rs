mod components;
mod pancurses_color;
mod systems;

use std::{collections::HashMap, ops::Range, time::Duration};

use antigen::{
    components::ChildEntitiesComponent,
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
    ecs::entity_component_database::SingleThreadedDatabase,
    ecs::system_runner::SingleThreadedSystemRunner,
    ecs::ComponentID,
    ecs::EntityComponentSystem,
    ecs::HeapComponentStorage,
    ecs::SystemRunner,
    ecs::{Assemblage, EntityComponentDatabase, EntityID, SystemError},
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
    pancurses_color_set_component::PancursesColorSetComponent,
    pancurses_input_axis_component::PancursesInputAxisComponent,
    pancurses_input_buffer_component::PancursesInputBufferComponent,
    pancurses_mouse_component::PancursesMouseComponent,
    pancurses_window_component::PancursesWindowComponent,
};
use pancurses_color::{PancursesColor, PancursesColorPair};
use systems::{
    DestructionTestInputSystem, InputVelocitySystem, ListSystem, LocalMousePositionSystem,
    PancursesInputAxisSystem, PancursesInputSystem, PancursesRendererSystem, PancursesWindowSystem,
};

// TODO: Factor boxed component storage out of EntityComponentDatabase, rename to EntityComponentDirectory
//       Directory should only care about storing IDs and doing lookups on them
//       Storage should be a separate object that cares about instantiating ComponentData structs and handing out &dyn ComponentData / &mut dyn ComponentData pointers when access is required
//       May need some super-object to manage Storage + Directory + SystemRunner

// TODO: Event synthesis for list hover / click
//       Global 'event queue' entity w/component containing array of boxed event traits (heap allocating on every event seems like a bad idea)
//       Emitter systems push events into the array
//       Receiver systems iterate through the array, respond to, and optionally consume events
//       How to avoid memory leaks from un-consumed events? Clear at beginning of frame and assume emitters will always run before receivers?

// TODO: Click to set inspected entity / component

// TODO: System list window
//       Will need a way for a new DebugSystemListComponent to know about registered systems
//       Should the Entity/Component DB or SystemRunner take care of this?

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

fn create_string_control(
    db: &mut impl EntityComponentDatabase,
    string_assemblage: &Assemblage,
    debug_label: Option<&str>,
    text: &str,
    (x, y): (i64, i64),
) -> Result<EntityID, String> {
    let entity_id = string_assemblage.create_and_assemble_entity(db, debug_label)?;

    db.get_entity_component_mut::<StringComponent>(entity_id)?
        .set_data(text.into());

    db.get_entity_component_mut::<PositionComponent>(entity_id)?
        .set_position(IVector2(x, y));

    Ok(entity_id)
}

fn create_window_entity(
    db: &mut impl EntityComponentDatabase,
    debug_label: Option<&str>,
    position: IVector2,
    size: IVector2,
    parent_window_entity_id: Option<EntityID>,
) -> Result<EntityID, String> {
    let entity_id = db.create_entity(debug_label)?;
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
) -> Result<HashMap<EntityAssemblage, Assemblage>, String> {
    let mut assemblages: HashMap<EntityAssemblage, Assemblage> = HashMap::new();

    assemblages.insert(
        EntityAssemblage::Player,
        Assemblage::build(
            db,
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
        Assemblage::build(db, "String Entity", "ASCII string control")
            .add_component(ControlComponent)?
            .add_component(StringComponent::default())?
            .add_component(PositionComponent::default())?
            .finish(),
    );

    assemblages.insert(
        EntityAssemblage::RectControl,
        Assemblage::build(db, "Rect Entity", "ASCII Rectangle control")
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
        Assemblage::build(db, "Border Entity", "ASCII Border control")
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
            db,
            "Destruction Test",
            "Assemblage for destroying entities when space is pressed",
        )
        .add_component(DestructionTestInputComponent::new(' '))?
        .add_component(PancursesInputBufferComponent::default())?
        .finish(),
    );

    Ok(assemblages)
}

fn create_game_window(
    db: &mut impl EntityComponentDatabase,
    assemblages: &HashMap<EntityAssemblage, Assemblage>,
    parent_window_entity: EntityID,
) -> Result<EntityID, String> {
    // Create Game Window
    let game_window_entity = db.create_entity(Some("Game"))?;
    db.add_component_to_entity(game_window_entity, PositionComponent::default())?;
    db.add_component_to_entity(game_window_entity, SizeComponent::default())?;
    db.add_component_to_entity(
        game_window_entity,
        ParentEntityComponent::new(parent_window_entity),
    )?;

    db.add_component_to_entity(
        game_window_entity,
        AnchorsComponent::new(0.0..0.5, 0.0..1.0),
    )?;

    // Create Test Player
    let test_player_entity = assemblages[&EntityAssemblage::Player]
        .create_and_assemble_entity(db, Some("Test Player"))?;
    db.add_component_to_entity(
        test_player_entity,
        ParentEntityComponent::new(game_window_entity),
    )?;
    db.add_component_to_entity(test_player_entity, GlobalPositionComponent::default())?;

    // Create Test String
    let test_string_entity = assemblages[&EntityAssemblage::StringControl]
        .create_and_assemble_entity(db, Some("Test String Control"))?;
    db.get_entity_component_mut::<PositionComponent>(test_string_entity)?
        .set_position(IVector2(1, 1));
    db.get_entity_component_mut::<StringComponent>(test_string_entity)?
        .set_data("Testing One Two Three".into());
    db.add_component_to_entity(
        test_string_entity,
        ParentEntityComponent::new(test_player_entity),
    )?;
    db.add_component_to_entity(test_string_entity, GlobalPositionComponent::default())?;

    // Create Test Rect
    let test_rect_entity = assemblages[&EntityAssemblage::RectControl]
        .create_and_assemble_entity(db, Some("Test Rect Control"))?;

    db.get_entity_component_mut::<PositionComponent>(test_rect_entity)?
        .set_position(IVector2(1, 5));

    db.get_entity_component_mut::<SizeComponent>(test_rect_entity)?
        .set_size(IVector2(20, 5));
    db.add_component_to_entity(
        test_rect_entity,
        ParentEntityComponent::new(test_player_entity),
    )?;
    db.add_component_to_entity(test_rect_entity, GlobalPositionComponent::default())?;

    Ok(game_window_entity)
}

fn create_debug_window(
    db: &mut impl EntityComponentDatabase,
    assemblages: &HashMap<EntityAssemblage, Assemblage>,
    parent_window_entity: EntityID,
    list_index_entity: Option<EntityID>,
    window_name: &str,
    anchor_horizontal: Range<f32>,
    anchor_vertical: Range<f32>,
) -> Result<EntityID, String> {
    let entity_list_window_entity = assemblages[&EntityAssemblage::RectControl]
        .create_and_assemble_entity(db, Some(&format!("{} Window", window_name)))?;
    db.get_entity_component_mut::<PancursesColorPairComponent>(entity_list_window_entity)?
        .set_data(PancursesColorPair::new(
            PancursesColor::new(0, 0, 0),
            PancursesColor::new(0, 0, 0),
        ));
    db.add_component_to_entity(entity_list_window_entity, PositionComponent::default())?;
    db.add_component_to_entity(entity_list_window_entity, ZIndexComponent::new(1))?;
    db.add_component_to_entity(entity_list_window_entity, SizeComponent::default())?;
    db.add_component_to_entity(
        entity_list_window_entity,
        ParentEntityComponent::new(parent_window_entity),
    )?;
    db.add_component_to_entity(
        entity_list_window_entity,
        AnchorsComponent::new(anchor_horizontal, anchor_vertical),
    )?;

    let entity_list_border_entity = assemblages[&EntityAssemblage::BorderControl]
        .create_and_assemble_entity(db, Some(&format!("{} Border", window_name)))?;
    db.add_component_to_entity(
        entity_list_border_entity,
        ParentEntityComponent::new(entity_list_window_entity),
    )?;
    db.add_component_to_entity(
        entity_list_border_entity,
        AnchorsComponent::new(0.0..1.0, 0.0..1.0),
    )?;

    // Create Debug Window Title
    let entity_list_title_entity = create_string_control(
        db,
        &assemblages[&EntityAssemblage::StringControl],
        Some(&format!("{} Title", window_name)),
        &format!("{}\n========", window_name),
        (2, 1),
    )?;
    db.add_component_to_entity(
        entity_list_title_entity,
        ParentEntityComponent::new(entity_list_border_entity),
    )?;
    db.add_component_to_entity(entity_list_title_entity, GlobalPositionComponent::default())?;

    // Create Entity List
    let entity_list_entity = db.create_entity(Some(window_name))?;
    db.add_component_to_entity(
        entity_list_entity,
        ListComponent::new(Some(entity_list_entity), list_index_entity, None),
    )?;
    db.add_component_to_entity(entity_list_entity, PositionComponent::default())?;
    db.add_component_to_entity(entity_list_entity, SizeComponent::default())?;
    db.add_component_to_entity(
        entity_list_entity,
        ParentEntityComponent::new(entity_list_border_entity),
    )?;
    db.add_component_to_entity(
        entity_list_entity,
        AnchorsComponent::new(0.0..1.0, 0.0..1.0),
    )?;
    db.add_component_to_entity(entity_list_entity, MarginsComponent::new(2, 2, 3, 1))?;
    db.add_component_to_entity(entity_list_entity, StringListComponent::default())?;
    db.add_component_to_entity(entity_list_entity, LocalMousePositionComponent::default())?;
    db.add_component_to_entity(entity_list_entity, DebugExcludeComponent)?;

    Ok(entity_list_entity)
}

fn create_entity_list_window(
    db: &mut impl EntityComponentDatabase,
    assemblages: &HashMap<EntityAssemblage, Assemblage>,
    parent_window_entity: EntityID,
    entity_inspector_entity: EntityID,
) -> Result<EntityID, String> {
    let entity_list_entity = create_debug_window(
        db,
        assemblages,
        parent_window_entity,
        Some(entity_inspector_entity),
        "Entities",
        0.25..0.5,
        0.0..0.5,
    )?;
    db.add_component_to_entity(entity_list_entity, DebugEntityListComponent)?;
    Ok(entity_list_entity)
}

fn create_component_list_window(
    db: &mut impl EntityComponentDatabase,
    assemblages: &HashMap<EntityAssemblage, Assemblage>,
    parent_window_entity: EntityID,
    component_inspector_entity: EntityID,
) -> Result<EntityID, String> {
    let component_list_entity = create_debug_window(
        db,
        assemblages,
        parent_window_entity,
        Some(component_inspector_entity),
        "Components",
        0.5..0.75,
        0.0..1.0,
    )?;
    db.add_component_to_entity(component_list_entity, DebugComponentListComponent)?;
    db.add_component_to_entity(component_list_entity, DebugExcludeComponent)?;

    Ok(component_list_entity)
}

fn create_component_data_list_window(
    db: &mut impl EntityComponentDatabase,
    assemblages: &HashMap<EntityAssemblage, Assemblage>,
    parent_window_entity: EntityID,
) -> Result<EntityID, String> {
    let component_list_entity = create_debug_window(
        db,
        assemblages,
        parent_window_entity,
        None,
        "Component Data",
        0.75..1.0,
        0.0..1.0,
    )?;
    db.add_component_to_entity(component_list_entity, DebugComponentDataListComponent)?;
    Ok(component_list_entity)
}

fn main_internal() -> Result<(), SystemError> {
    let mut storage = HeapComponentStorage::new();

    /*
    control_component::ControlComponent,
    destruction_test_input_component::DestructionTestInputComponent, fill_component::FillComponent,
    list_component::ListComponent, local_mouse_position_component::LocalMousePositionComponent,
    pancurses_color_pair_component::PancursesColorPairComponent,
    pancurses_input_buffer_component::PancursesInputBufferComponent,
    pancurses_window_component::PancursesWindowComponent,
    */

    storage.register_component::<AnchorsComponent>();
    storage.register_component::<CharComponent>();
    storage.register_component::<ComponentDebugComponent>();
    storage.register_component::<ComponentInspectorComponent>();
    storage.register_component::<ChildEntitiesComponent>();
    storage.register_component::<DebugComponentDataListComponent>();
    storage.register_component::<DebugComponentListComponent>();
    storage.register_component::<DebugEntityListComponent>();
    storage.register_component::<DebugExcludeComponent>();
    storage.register_component::<EntityDebugComponent>();
    storage.register_component::<EntityInspectorComponent>();
    storage.register_component::<GlobalPositionComponent>();
    storage.register_component::<IntRangeComponent>();
    storage.register_component::<MarginsComponent>();
    storage.register_component::<ParentEntityComponent>();
    storage.register_component::<PositionComponent>();
    storage.register_component::<SizeComponent>();
    storage.register_component::<StringComponent>();
    storage.register_component::<StringListComponent>();
    storage.register_component::<VelocityComponent>();
    storage.register_component::<WindowComponent>();
    storage.register_component::<ZIndexComponent>();

    storage.register_component::<ControlComponent>();
    storage.register_component::<DestructionTestInputComponent>();
    storage.register_component::<FillComponent>();
    storage.register_component::<ListComponent>();
    storage.register_component::<LocalMousePositionComponent>();
    storage.register_component::<PancursesColorPairComponent>();
    storage.register_component::<PancursesInputAxisComponent>();
    storage.register_component::<PancursesInputBufferComponent>();
    storage.register_component::<PancursesMouseComponent>();
    storage.register_component::<PancursesWindowComponent>();
    storage.register_component::<PancursesColorSetComponent>();

    println!("Storage: {:#?}", storage);

    let mut pancurses_window_system = PancursesWindowSystem::new(&mut storage);
    let mut pancurses_input_system = PancursesInputSystem::new(1);
    let mut pancurses_prev_next_input_system = PancursesInputAxisSystem::new();
    let mut destruction_test_input_system = DestructionTestInputSystem::new();
    let mut local_mouse_position_system = LocalMousePositionSystem::new();
    let mut list_system = ListSystem::new();
    let mut input_velocity_system = InputVelocitySystem::new();
    let mut position_integrator_system = PositionIntegratorSystem::new();
    let mut anchors_margins_system = AnchorsMarginsSystem::new();
    let mut global_position_system = GlobalPositionSystem::new();

    let mut db = SingleThreadedDatabase::new(&mut storage)?;

    let mut ecs_debug_system = ECSDebugSystem::new(&mut db);
    let mut child_entities_system = ChildEntitiesSystem::new();
    let mut pancurses_renderer_system = PancursesRendererSystem::new();

    let assemblages = setup_assemblages(&mut db)?;

    // Create Main Window
    let main_window_entity = create_window_entity(
        &mut db,
        Some("Main Window"),
        IVector2::default(),
        IVector2(256, 64),
        None,
    )?;

    let entity_inspector_entity = db.create_entity(Some("Entity Inspector"))?;
    db.add_component_to_entity(entity_inspector_entity, EntityInspectorComponent)?;
    db.add_component_to_entity(entity_inspector_entity, IntRangeComponent::default())?;

    let component_inspector_entity = db.create_entity(Some("Component Inspector"))?;
    db.add_component_to_entity(component_inspector_entity, ComponentInspectorComponent)?;
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
    create_component_data_list_window(&mut db, &assemblages, main_window_entity)?;

    // Create systems
    let mut system_runner = SingleThreadedSystemRunner::<SingleThreadedDatabase>::new();
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
    system_runner.register_system("Anchors Margins", &mut anchors_margins_system);
    system_runner.register_system("Global Position", &mut global_position_system);
    system_runner.register_system("Child Entities", &mut child_entities_system);
    system_runner.register_system("Pancurses Renderer", &mut pancurses_renderer_system);

    // Main loop
    let frame_time_target = Duration::from_secs_f32(1.0 / 60.0);
    loop {
        let main_loop_profiler = Profiler::start("Main Loop");

        system_runner.run(&mut db)?;

        // Sleep if framerate target is exceeded - prevents deadlock when pancurses stops being able to poll input after window close
        let delta = main_loop_profiler.finish();
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
