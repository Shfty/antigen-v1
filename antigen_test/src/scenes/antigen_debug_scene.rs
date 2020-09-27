use std::{collections::HashMap, ops::Range};

use antigen::{
    components::CPUShader,
    components::Control,
    components::DebugSceneTree,
    components::DebugSystemList,
    components::List,
    components::LocalPosition,
    components::SoftwareFramebuffer,
    components::SystemInspector,
    components::{
        Anchors, ComponentInspector, DebugComponentDataList, DebugComponentList, DebugEntityList,
        DebugExclude, EntityInspector, GlobalPosition, IntRange, Margins, ParentEntity, Position,
        Size, Velocity, Window, ZIndex,
    },
    core::palette::RGBArrangementPalette,
    entity_component_system::ComponentStorage,
    entity_component_system::EntityComponentDirectory,
    entity_component_system::Scene,
    entity_component_system::{
        system_interface::SystemInterface, system_storage::SystemStorage, Assemblage,
        EntityComponentSystem, EntityID, SystemRunner,
    },
    primitive_types::Color,
    primitive_types::ColorRGBF,
    primitive_types::Vector2I,
    systems::AntigenInputEventQueueSystem,
    systems::{
        AnchorsMarginsSystem, ChildEntitiesSystem, GlobalPositionSystem, ListSystem,
        LocalMousePositionSystem, PositionIntegratorSystem, SoftwareRendererSystem,
        StringRendererSystem,
    },
};
use antigen_curses::{
    CursesEventQueueSystem, CursesInputBufferSystem, CursesKeyboardSystem, CursesMouseSystem,
    CursesRendererSystem, CursesWindow, CursesWindowSystem, TextColorMode,
};

use crate::systems::{DestructionTestInputSystem, InputAxisSystem, InputVelocitySystem};
use crate::{components::DestructionTestInput, systems::QuitKeySystem};

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
        ecs.push_system(AntigenInputEventQueueSystem::new());
        ecs.push_system(CursesEventQueueSystem::new());

        ecs.push_system(CursesInputBufferSystem);
        ecs.push_system(CursesKeyboardSystem);
        ecs.push_system(CursesMouseSystem::new());
        let pancurses_window_system = CursesWindowSystem::new(&mut ecs.component_storage);
        ecs.push_system(pancurses_window_system);

        ecs.push_system(QuitKeySystem::new(antigen::core::keyboard::Key::Escape));
        ecs.push_system(InputAxisSystem);
        ecs.push_system(DestructionTestInputSystem::new());
        ecs.push_system(LocalMousePositionSystem::new());
        ecs.push_system(ListSystem::new());
        ecs.push_system(InputVelocitySystem::new());

        ecs.push_system(PositionIntegratorSystem::new());
        ecs.push_system(AnchorsMarginsSystem::new());
        ecs.push_system(GlobalPositionSystem::new());
        ecs.push_system(ChildEntitiesSystem::new());
        ecs.push_system(SoftwareRendererSystem);
        ecs.push_system(StringRendererSystem);
        ecs.push_system(CursesRendererSystem::new(
            RGBArrangementPalette::new_884(),
            TextColorMode::BlackWhite,
        ));

        Ok(())
    }

    fn create_entities<CS, CD>(db: &mut SystemInterface<CS, CD>) -> Result<(), String>
    where
        CS: ComponentStorage,
        CD: EntityComponentDirectory,
    {
        let mut assemblages = create_assemblages()?;

        // Create main window
        let cpu_framebuffer_entity = db.create_entity("CPU Framebuffer".into())?;
        db.insert_entity_component(
            cpu_framebuffer_entity,
            SoftwareFramebuffer::new(Color(0.0f32, 0.0f32, 0.0f32)),
        )?;

        let string_framebuffer_entity = db.create_entity("String Framebuffer".into())?;
        db.insert_entity_component(string_framebuffer_entity, SoftwareFramebuffer::new(' '))?;

        let main_window_entity = create_window_entity(
            db,
            Some("Main Window"),
            Position::default(),
            Size::from(Vector2I(256, 64)),
            None,
        )?;

        let entity_inspector_entity = db.create_entity(Some("Entity Inspector"))?;
        db.insert_entity_component(entity_inspector_entity, EntityInspector)?;
        db.insert_entity_component(entity_inspector_entity, IntRange::new(-1..-1))?;

        let component_inspector_entity = db.create_entity(Some("Component Inspector"))?;
        db.insert_entity_component(component_inspector_entity, ComponentInspector)?;
        db.insert_entity_component(component_inspector_entity, IntRange::new(-1..-1))?;

        let system_inspector_entity = db.create_entity(Some("System Inspector"))?;
        db.insert_entity_component(system_inspector_entity, SystemInspector)?;
        db.insert_entity_component(system_inspector_entity, IntRange::new(-1..-1))?;

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

    fn load<'a, CS, CD, SS, SR>(
        ecs: &'a mut EntityComponentSystem<CS, CD, SS, SR>,
    ) -> Result<(), String>
    where
        CS: ComponentStorage,
        CD: EntityComponentDirectory + 'static,
        SS: SystemStorage<CS, CD> + 'static,
        SR: SystemRunner + 'static,
    {
        Self::register_systems(ecs)?;
        let mut entity_component_database = ecs.get_system_interface();
        Self::create_entities(&mut entity_component_database)?;
        Ok(())
    }
}

fn create_string_control<CS, CD>(
    db: &mut SystemInterface<CS, CD>,
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

    *db.get_entity_component_mut::<String>(entity_id)? = text.into();

    *db.get_entity_component_mut::<Position>(entity_id)? = Vector2I(x, y).into();

    Ok(entity_id)
}

fn create_window_entity<S, D>(
    db: &mut SystemInterface<S, D>,
    debug_label: Option<&str>,
    position: Position,
    size: Size,
    parent_window_entity_id: Option<EntityID>,
) -> Result<EntityID, String>
where
    S: ComponentStorage,
    D: EntityComponentDirectory,
{
    let entity_id = db.create_entity(debug_label)?;
    db.insert_entity_component(entity_id, Window)?;
    db.insert_entity_component(entity_id, CursesWindow::default())?;
    db.insert_entity_component(entity_id, position)?;
    db.insert_entity_component(entity_id, size)?;
    if let Some(parent_window_entity_id) = parent_window_entity_id {
        db.insert_entity_component(entity_id, ParentEntity(parent_window_entity_id))?;
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
        .add_component(Control)?
        .add_component(Color(1.0f32, 0.6f32, 1.0f32))?
        .add_component('@')?
        .add_component(Position::from(Vector2I(1, 1)))?
        .add_component(Velocity::default())?
        .finish(),
    );

    assemblages.insert(
        EntityAssemblage::StringControl,
        Assemblage::build("String Entity", "ASCII string control")
            .add_component(Control)?
            .add_component(String::default())?
            .add_component(Position::default())?
            .finish(),
    );

    assemblages.insert(
        EntityAssemblage::RectControl,
        Assemblage::build("Rect Entity", "ASCII Rectangle control")
            .add_component(Control)?
            .add_component(Position::default())?
            .add_component(Size::default())?
            .add_component(char::default())?
            .add_component(Color(0.753f32, 0.753f32, 0.753f32))?
            .finish(),
    );

    assemblages.insert(
        EntityAssemblage::BorderControl,
        Assemblage::build("Border Entity", "ASCII Border control")
            .add_component(Control)?
            .add_component(Position::default())?
            .add_component(Size::default())?
            .add_component(char::default())?
            .add_component(CPUShader(CPUShader::rect))?
            .add_component(Color(0.753f32, 0.753f32, 0.753f32))?
            .finish(),
    );

    assemblages.insert(
        EntityAssemblage::DestructionTest,
        Assemblage::build(
            "Destruction Test",
            "Assemblage for destroying entities when space is pressed",
        )
        .add_component(DestructionTestInput(antigen::core::keyboard::Key::Space))?
        .finish(),
    );

    Ok(assemblages)
}

fn create_game_window<CS, CD>(
    db: &mut SystemInterface<CS, CD>,
    assemblages: &mut HashMap<EntityAssemblage, Assemblage<CS, CD>>,
    parent_window_entity: EntityID,
) -> Result<EntityID, String>
where
    CS: ComponentStorage,
    CD: EntityComponentDirectory,
{
    // Create Game Window
    let game_window_entity = db.create_entity(Some("Game"))?;
    db.insert_entity_component(game_window_entity, Position::default())?;
    db.insert_entity_component(game_window_entity, Size::default())?;
    db.insert_entity_component(game_window_entity, ParentEntity(parent_window_entity))?;

    db.insert_entity_component(game_window_entity, Anchors::new(0.0..0.5, 0.0..1.0))?;

    // Create Test Rects
    for (position, shader, color) in [
        (
            Vector2I(1, 5),
            CPUShader(CPUShader::uv),
            Color(1.0, 1.0, 1.0),
        ),
        (
            Vector2I(1, 12),
            CPUShader(CPUShader::gradient_horizontal),
            Color(1.0, 1.0, 1.0),
        ),
        (
            Vector2I(1, 19),
            CPUShader(CPUShader::gradient_horizontal),
            Color(1.0, 0.0, 0.0),
        ),
        (
            Vector2I(1, 26),
            CPUShader(CPUShader::gradient_horizontal),
            Color(0.0, 1.0, 0.0),
        ),
        (
            Vector2I(1, 33),
            CPUShader(CPUShader::gradient_horizontal),
            Color(0.0, 0.0, 1.0),
        ),
        (
            Vector2I(1, 40),
            CPUShader(CPUShader::gradient_horizontal),
            Color(0.0, 1.0, 1.0),
        ),
        (
            Vector2I(1, 47),
            CPUShader(CPUShader::gradient_horizontal),
            Color(1.0, 0.0, 1.0),
        ),
        (
            Vector2I(1, 54),
            CPUShader(CPUShader::gradient_horizontal),
            Color(1.0, 1.0, 0.0),
        ),
    ]
    .iter()
    {
        let test_rect_entity = assemblages
            .get_mut(&EntityAssemblage::RectControl)
            .unwrap()
            .create_and_assemble_entity(db, Some("Test Rect Control"))?;
        {
            *db.get_entity_component_mut::<Position>(test_rect_entity)? = (*position).into();
            *db.get_entity_component_mut::<Size>(test_rect_entity)? = Vector2I(48, 6).into();

            db.insert_entity_component(test_rect_entity, ParentEntity(game_window_entity))?;
            db.insert_entity_component(test_rect_entity, GlobalPosition::default())?;
            db.insert_entity_component(test_rect_entity, *shader)?;
            db.insert_entity_component(test_rect_entity, *color)?;
        }
    }

    // Create Test Player
    let test_player_entity = assemblages
        .get_mut(&EntityAssemblage::Player)
        .unwrap()
        .create_and_assemble_entity(db, Some("Test Player"))?;
    db.insert_entity_component(test_player_entity, ParentEntity(game_window_entity))?;
    db.insert_entity_component(test_player_entity, GlobalPosition::default())?;

    // Create Test String
    let test_string_entity = assemblages
        .get_mut(&EntityAssemblage::StringControl)
        .unwrap()
        .create_and_assemble_entity(db, Some("Test String Control"))?;
    {
        *db.get_entity_component_mut::<Position>(test_string_entity)? = Vector2I(1, 1).into();
        *db.get_entity_component_mut::<String>(test_string_entity)? =
            "Testing One Two Three".into();

        db.insert_entity_component(test_string_entity, ParentEntity(test_player_entity))?;
        db.insert_entity_component(test_string_entity, GlobalPosition::default())?;
    }

    Ok(game_window_entity)
}

fn create_debug_window<CS, CD>(
    db: &mut SystemInterface<CS, CD>,
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
    {
        *db.get_entity_component_mut::<ColorRGBF>(entity_list_window_entity)? =
            Color(0.0, 0.0, 0.0);

        db.insert_entity_component(entity_list_window_entity, Position::default())?;
        db.insert_entity_component(entity_list_window_entity, ZIndex(1))?;
        db.insert_entity_component(entity_list_window_entity, Size::default())?;
        db.insert_entity_component(
            entity_list_window_entity,
            ParentEntity(parent_window_entity),
        )?;
        db.insert_entity_component(
            entity_list_window_entity,
            Anchors::new(anchor_horizontal, anchor_vertical),
        )?;
    }

    let entity_list_border_entity = assemblages
        .get_mut(&EntityAssemblage::BorderControl)
        .unwrap()
        .create_and_assemble_entity(db, Some(&format!("{} Border", window_name)))?;
    {
        db.insert_entity_component(
            entity_list_border_entity,
            ParentEntity(entity_list_window_entity),
        )?;
        db.insert_entity_component(entity_list_border_entity, Anchors::new(0.0..1.0, 0.0..1.0))?;
    }

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
    {
        db.insert_entity_component(
            entity_list_title_entity,
            ParentEntity(entity_list_border_entity),
        )?;
        db.insert_entity_component(entity_list_title_entity, GlobalPosition::default())?;
    }

    // Create Entity List
    let entity_list_entity = db.create_entity(Some(window_name))?;
    {
        let list_component = List::new(Some(entity_list_entity), list_index_entity);

        db.insert_entity_component(entity_list_entity, list_component)?;
        db.insert_entity_component(entity_list_entity, Position::default())?;
        db.insert_entity_component(entity_list_entity, Size::default())?;
        db.insert_entity_component(entity_list_entity, ParentEntity(entity_list_border_entity))?;
        db.insert_entity_component(entity_list_entity, Anchors::new(0.0..1.0, 0.0..1.0))?;
        db.insert_entity_component(entity_list_entity, Margins::new(2, 2, 3, 1))?;
        db.insert_entity_component(entity_list_entity, Vec::new())?;
        db.insert_entity_component(entity_list_entity, LocalPosition::default())?;
        db.insert_entity_component(entity_list_entity, DebugExclude)?;
    }

    Ok(entity_list_entity)
}

fn create_entity_list_window<S, D>(
    db: &mut SystemInterface<S, D>,
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
    db.insert_entity_component(entity_list_entity, DebugEntityList)?;
    Ok(entity_list_entity)
}

fn create_scene_tree_window<S, D>(
    db: &mut SystemInterface<S, D>,
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
        None,
        "Scene Tree",
        0.25..0.5,
        0.5..1.0,
    )?;
    db.insert_entity_component(entity_list_entity, DebugSceneTree)?;
    Ok(entity_list_entity)
}

fn create_component_list_window<S, D>(
    db: &mut SystemInterface<S, D>,
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
    db.insert_entity_component(component_list_entity, DebugComponentList)?;
    db.insert_entity_component(component_list_entity, DebugExclude)?;

    Ok(component_list_entity)
}

fn create_component_data_list_window<S, D>(
    db: &mut SystemInterface<S, D>,
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
    db.insert_entity_component(component_list_entity, DebugComponentDataList)?;
    Ok(component_list_entity)
}

fn create_system_list_window<S, D>(
    db: &mut SystemInterface<S, D>,
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
    db.insert_entity_component(system_list_entity, DebugSystemList)?;
    Ok(system_list_entity)
}
