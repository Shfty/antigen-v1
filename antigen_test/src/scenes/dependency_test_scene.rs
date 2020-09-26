use antigen::{
    components::ColorComponent,
    components::{
        CharComponent, ControlComponent, ParentEntityComponent, PositionComponent, SizeComponent,
        VelocityComponent, WindowComponent,
    },
    entity_component_system::ComponentStorage,
    entity_component_system::EntityComponentDirectory,
    entity_component_system::Scene,
    entity_component_system::SystemInterface,
    entity_component_system::{
        system_storage::SystemStorage, Assemblage, EntityComponentSystem, SystemRunner,
    },
    palette::RGBArrangementPalette,
    primitive_types::Color,
    primitive_types::Vector2I,
    systems::PositionIntegratorSystem,
};
use antigen_curses::{
    CursesInputBufferSystem, CursesRendererSystem, CursesWindowComponent, CursesWindowSystem,
    TextColorMode,
};

use crate::systems::InputVelocitySystem;
use crate::systems::QuitKeySystem;

pub struct DependencyTestScene;

impl Scene for DependencyTestScene {
    fn register_systems<CS, CD, SS, SR>(
        ecs: &mut EntityComponentSystem<CS, CD, SS, SR>,
    ) -> Result<(), String>
    where
        CS: ComponentStorage,
        CD: EntityComponentDirectory + 'static,
        SS: SystemStorage<CS, CD> + 'static,
        SR: SystemRunner + 'static,
    {
        // Resolution Strategy
        // Treat predicates as ref fallbacks for components that don't get read or written (ex. WindowComponent)
        // For each mutable component reference in a given system
        //   If no other systems take a mutable reference to the same component, this system is the component's entrypoint
        //   If any other system takes a mutable reference to the same component, the relation should be ignored and dependencies inferred from other components
        //   Otherwise, non-mutable references to the same component should be checked and stored as a System > System map

        // pred: (WindowComponent, PancursesWindowComponent, SizeComponent)
        // ref: PancursesWindowComponent, SizeComponent, CharComponent, PancursesColorPairComponent, StringComponent
        // mut: SizeComponent, PancursesColorSetComponent, PancursesWindowComponent
        let pancurses_window_system = CursesWindowSystem::new(&mut ecs.component_storage);
        ecs.push_system(pancurses_window_system);

        // pred: (WindowComponent, PancursesWindowComponent)
        // ref: PancursesWindowComponent
        // mut: ?MouseComponent, EventQueueComponent<AntigenEvent>
        ecs.push_system(CursesInputBufferSystem::new(1));

        ecs.push_system(QuitKeySystem::new(antigen::keyboard::Key::Escape));

        // pred: VelocityComponent
        // ref: EventQueueComponent<AntigenEvent>
        // mut: VelocityComponent
        ecs.push_system(InputVelocitySystem::new());

        // pred: (PositionComponent, VelocityComponent)
        // ref: VelocityComponent
        // mut: PositionComponent
        ecs.push_system(PositionIntegratorSystem::new());

        // pred: PancursesColorSetComponent, (ControlComponent, ParentEntityComponent, PositionComponent), (WindowComponent, PancursesWindowComponent, SizeComponent)
        // ref: ParentEntityComponent, ZIndexComponent, ChildEntitiesComponent, PancursesWindowComponent, ParentEntityComponent, PancursesWindowComponent,
        //      ParentEntityComponent, GlobalPositionComponent, PositionComponent, PancursesColorPairComponent, CharComponent, SizeComponent, StringComponent, PancursesWindowComponent
        // mut: PancursesColorSetComponent
        ecs.push_system(CursesRendererSystem::new(
            RGBArrangementPalette::new_884(),
            TextColorMode::Color(Color(0.0, 0.0, 0.0)),
        ));

        Ok(())
    }

    fn create_entities<CS, CD>(db: &mut SystemInterface<CS, CD>) -> Result<(), String>
    where
        CS: ComponentStorage,
        CD: EntityComponentDirectory,
    {
        // Create Main Window
        let main_window_entity = db.create_entity(Some("Main Window"))?;
        {
            db.insert_entity_component(main_window_entity, WindowComponent)?;
            db.insert_entity_component(main_window_entity, CursesWindowComponent::default())?;
            db.insert_entity_component(main_window_entity, SizeComponent::new(Vector2I(64, 32)))?;
        }

        // Create Player
        let mut player_assemblage = Assemblage::build(
            "Player Entity",
            "Controllable ASCII character with position and velocity",
        )
        .add_component(ControlComponent)?
        .add_component(PositionComponent::new(Vector2I(1, 1)))?
        .add_component(VelocityComponent::new(Vector2I(1, 1)))?
        .add_component(CharComponent::new('@'))?
        .add_component(ColorComponent::new(Color(1.0, 0.6, 1.0)))?
        .finish();

        let test_player_entity =
            player_assemblage.create_and_assemble_entity(db, Some("Test Player"))?;
        {
            db.insert_entity_component(
                test_player_entity,
                ParentEntityComponent::new(main_window_entity),
            )?;
        }

        Ok(())
    }
}
