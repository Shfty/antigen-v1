use antigen::systems::EventQueueSystem;

use crate::CursesEvent;

pub type CursesEventQueueSystem = EventQueueSystem<CursesEvent>;
