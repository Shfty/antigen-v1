use std::{collections::HashMap, ops::Range};

use antigen::{
    components::DebugSceneTreeComponent,
    components::DebugSystemListComponent,
    components::SystemInspectorComponent,
    components::{
        AnchorsComponent, CharComponent, ComponentInspectorComponent,
        DebugComponentDataListComponent, DebugComponentListComponent, DebugEntityListComponent,
        DebugExcludeComponent, EntityInspectorComponent, GlobalPositionComponent,
        IntRangeComponent, MarginsComponent, ParentEntityComponent, PositionComponent,
        SizeComponent, StringComponent, StringListComponent, VelocityComponent, WindowComponent,
        ZIndexComponent,
    },
    entity_component_system::Scene,
    entity_component_system::{
        entity_component_database::{
            ComponentStorage, EntityComponentDatabase, EntityComponentDirectory,
        },
        system_storage::SystemStorage,
        Assemblage, EntityComponentSystem, EntityID, SystemRunner,
    },
    primitive_types::IVector2,
    systems::{
        AnchorsMarginsSystem, ChildEntitiesSystem, GlobalPositionSystem, PositionIntegratorSystem,
    },
};

use crate::components::{
    control_component::ControlComponent,
    destruction_test_input_component::DestructionTestInputComponent, fill_component::FillComponent,
    list_component::ListComponent, local_mouse_position_component::LocalMousePositionComponent,
    pancurses_color_pair_component::PancursesColorPairComponent,
    pancurses_input_buffer_component::PancursesInputBufferComponent,
    pancurses_window_component::PancursesWindowComponent,
};
use crate::pancurses_color::{PancursesColor, PancursesColorPair};
use crate::systems::{
    DestructionTestInputSystem, InputVelocitySystem, ListSystem, LocalMousePositionSystem,
    PancursesInputAxisSystem, PancursesInputSystem, PancursesRendererSystem, PancursesWindowSystem,
};

#[derive(Eq, PartialEq, Hash)]
enum EntityAssemblage {
    Player = 0,
    StringControl = 1,
    RectControl = 2,
    BorderControl = 3,
    DestructionTest = 4,
}

pub struct AntigenDebugScene;

impl Scene for AntigenDebugScene {
    fn register_systems<CS, CD, SS, SR>(
        ecs: &mut EntityComponentSystem<CS, CD, SS, SR>,
    ) -> Result<(), String>
    where
        CS: ComponentStorage,
        CD: EntityComponentDirectory + 'static,
        SS: SystemStorage<CS, CD> + 'static,
        SR: SystemRunner + 'static,
    {
        let pancurses_window_system = PancursesWindowSystem::new(&mut ecs.component_storage);
        ecs.push_system("Pancurses Window", pancurses_window_system);
        ecs.push_system("Pancurses Input", PancursesInputSystem::new(1));
        ecs.push_system("Pancurses Input Axis", PancursesInputAxisSystem::new());
        ecs.push_system("Destruction Test Input", DestructionTestInputSystem::new());
        ecs.push_system("Local Mouse Position", LocalMousePositionSystem::new());
        ecs.push_system("List", ListSystem::new());
        ecs.push_system("Input Velocity", InputVelocitySystem::new());
        ecs.push_system("Position Integrator", PositionIntegratorSystem::new());
        ecs.push_system("Anchors Margins", AnchorsMarginsSystem::new());
        ecs.push_system("Global Position", GlobalPositionSystem::new());
        ecs.push_system("Child Entities", ChildEntitiesSystem::new());
        ecs.push_system("Pancurses Renderer", PancursesRendererSystem::new());

        Ok(())
    }

    fn create_entities<CS, CD>(db: &mut EntityComponentDatabase<CS, CD>) -> Result<(), String>
    where
        CS: ComponentStorage,
        CD: EntityComponentDirectory,
    {
        let mut assemblages = create_assemblages()?;

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

        let system_inspector_entity = db.create_entity(Some("System Inspector"))?;
        db.insert_entity_component(system_inspector_entity, SystemInspectorComponent)?;
        db.insert_entity_component(system_inspector_entity, IntRangeComponent::default())?;

        create_game_window(db, &mut assemblages, main_window_entity)?;
        create_entity_list_window(
            db,
            &mut assemblages,
            main_window_entity,
            entity_inspector_entity,
        )?;
        create_scene_tree_window(
            db,
            &mut assemblages,
            main_window_entity,
            entity_inspector_entity,
        )?;
        create_component_list_window(
            db,
            &mut assemblages,
            main_window_entity,
            component_inspector_entity,
        )?;
        create_component_data_list_window(db, &mut assemblages, main_window_entity)?;
        create_system_list_window(
            db,
            &mut assemblages,
            main_window_entity,
            system_inspector_entity,
        )?;

        Ok(())
    }
}

fn create_string_control<CS, CD>(
    db: &mut EntityComponentDatabase<CS, CD>,
    string_assemblage: &mut Assemblage<CS, CD>,
    debug_label: Option<&str>,
    text: &str,
    (x, y): (i64, i64),
) -> Result<EntityID, String>
where
    CS: ComponentStorage,
    CD: EntityComponentDirectory,
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

fn create_assemblages<S, D>() -> Result<HashMap<EntityAssemblage, Assemblage<S, D>>, String>
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

fn create_game_window<CS, CD>(
    db: &mut EntityComponentDatabase<CS, CD>,
    assemblages: &mut HashMap<EntityAssemblage, Assemblage<CS, CD>>,
    parent_window_entity: EntityID,
) -> Result<EntityID, String>
where
    CS: ComponentStorage,
    CD: EntityComponentDirectory,
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
    {
        db.get_entity_component_mut::<PositionComponent>(test_string_entity)?
            .set_position(IVector2(1, 1));
        db.get_entity_component_mut::<StringComponent>(test_string_entity)?
            .set_data("Testing One Two Three".into());
        db.insert_entity_component(
            test_string_entity,
            ParentEntityComponent::new(test_player_entity),
        )?;
        db.insert_entity_component(test_string_entity, GlobalPositionComponent::default())?;
    }

    // Create Test Rect
    let test_rect_entity = assemblages
        .get_mut(&EntityAssemblage::RectControl)
        .unwrap()
        .create_and_assemble_entity(db, Some("Test Rect Control"))?;
    {
        db.get_entity_component_mut::<PositionComponent>(test_rect_entity)?
            .set_position(IVector2(1, 5));

        db.get_entity_component_mut::<SizeComponent>(test_rect_entity)?
            .set_size(IVector2(20, 5));
        db.insert_entity_component(
            test_rect_entity,
            ParentEntityComponent::new(test_player_entity),
        )?;
        db.insert_entity_component(test_rect_entity, GlobalPositionComponent::default())?;
    }

    Ok(game_window_entity)
}

fn create_debug_window<CS, CD>(
    db: &mut EntityComponentDatabase<CS, CD>,
    assemblages: &mut HashMap<EntityAssemblage, Assemblage<CS, CD>>,
    parent_window_entity: EntityID,
    list_index_entity: Option<EntityID>,
    window_name: &str,
    anchor_horizontal: Range<f32>,
    anchor_vertical: Range<f32>,
) -> Result<EntityID, String>
where
    CS: ComponentStorage,
    CD: EntityComponentDirectory,
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

fn create_scene_tree_window<S, D>(
    db: &mut EntityComponentDatabase<S, D>,
    assemblages: &mut HashMap<EntityAssemblage, Assemblage<S, D>>,
    parent_window_entity: EntityID,
    _entity_inspector_entity: EntityID,
) -> Result<EntityID, String>
where
    S: ComponentStorage,
    D: EntityComponentDirectory,
{
    let entity_list_entity = create_debug_window(
        db,
        assemblages,
        parent_window_entity,
        None,
        "Scene Tree",
        0.25..0.5,
        0.5..1.0,
    )?;
    db.insert_entity_component(entity_list_entity, DebugSceneTreeComponent)?;
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
        0.0..0.5,
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

fn create_system_list_window<S, D>(
    db: &mut EntityComponentDatabase<S, D>,
    assemblages: &mut HashMap<EntityAssemblage, Assemblage<S, D>>,
    parent_window_entity: EntityID,
    system_inspector_entity: EntityID,
) -> Result<EntityID, String>
where
    S: ComponentStorage,
    D: EntityComponentDirectory,
{
    let system_list_entity = create_debug_window(
        db,
        assemblages,
        parent_window_entity,
        Some(system_inspector_entity),
        "Systems",
        0.5..0.75,
        0.5..1.0,
    )?;
    db.insert_entity_component(system_list_entity, DebugSystemListComponent)?;
    Ok(system_list_entity)
}
