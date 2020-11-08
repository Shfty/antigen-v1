use crate::systems::InputVelocity;
use crate::systems::QuitKey;

use antigen::{
    assemblage::EntityBuilder,
    components::{Name, ParentEntity, Position, Size, StringShader, Velocity, Window},
    entity_component_system::{EntityID, SystemBuilder},
    primitive_types::{ColorRGB, Vector2I},
    systems::PositionIntegrator,
};
use antigen_curses::components::CursesWindowData;
use antigen_curses::systems as curses_systems;

pub struct DependencyTestScene;

pub fn system_assembler(builder: SystemBuilder) -> SystemBuilder {
    builder
        // Resolution Strategy
        // Treat predicates as ref fallbacks for components that don't get read or written (ex. WindowComponent)
        // For each mutable component reference in a given system
        //   If no other systems take a mutable reference to the same component, this system is the component's entrypoint
        //   If any other system takes a mutable reference to the same component, the relation should be ignored and dependencies inferred from other components
        //   Otherwise, non-mutable references to the same component should be checked and stored as a System > System map
        // pred: (WindowComponent, CursesWindowComponent, SizeComponent)
        // ref: CursesWindowComponent, SizeComponent, CharComponent, CursesColorPairComponent, StringComponent
        // mut: SizeComponent, CursesColorSetComponent, CursesWindowComponent
        .system(curses_systems::CursesWindow)
        // ???
        .system(curses_systems::CursesColors::new(
            curses_systems::TextColorMode::Color(ColorRGB(0.0, 0.0, 0.0)),
        ))
        // pred: (WindowComponent, CursesWindowComponent)
        // ref: CursesWindowComponent
        // mut: ?MouseComponent, EventQueueComponent<AntigenEvent>
        .system(curses_systems::CursesInputBuffer)
        .system(QuitKey::new(antigen::core::keyboard::Key::Escape))
        // pred: VelocityComponent
        // ref: EventQueueComponent<AntigenEvent>
        // mut: VelocityComponent
        .system(InputVelocity)
        // pred: (PositionComponent, VelocityComponent)
        // ref: VelocityComponent
        // mut: PositionComponent
        .system(PositionIntegrator)
        // pred: CursesColorSetComponent, (ControlComponent, ParentEntityComponent, PositionComponent), (WindowComponent, CursesWindowComponent, SizeComponent)
        // ref: ParentEntityComponent, ZIndexComponent, ChildEntitiesComponent, CursesWindowComponent, ParentEntityComponent, CursesWindowComponent,
        //      ParentEntityComponent, GlobalPositionComponent, PositionComponent, CursesColorPairComponent, CharComponent, SizeComponent, StringComponent, CursesWindowComponent
        // mut: CursesColorSetComponent
        .system(curses_systems::CursesRenderer::default())
}

pub fn entity_assembler(builder: EntityBuilder) -> EntityBuilder {
    builder
        .key(EntityID::next())
        .fields((
            Name("Main Window".into()),
            Window,
            CursesWindowData::default(),
            Size(Vector2I(64, 32)),
        ))
        .finish()
        .map(move |builder: EntityBuilder| {
            let main_window_entity = builder.current_key();

            builder
                .key(EntityID::next())
                .fields((
                    Name("Player Entity".into()),
                    StringShader,
                    Position(Vector2I::ONE),
                    Velocity::default(),
                    '@',
                    ColorRGB(1.0f32, 1.0f32, 1.0f32),
                    ParentEntity(main_window_entity),
                ))
                .finish()
        })
}
