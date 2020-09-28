use antigen::systems::GlobalEventQueueSystem;

use crate::CursesEvent;

pub type CursesEventQueueSystem = GlobalEventQueueSystem<CursesEvent>;
