use crate::{core::events::AntigenInputEvent, systems::GlobalEventQueueSystem};

pub type AntigenInputEventQueueSystem = GlobalEventQueueSystem<AntigenInputEvent>;
