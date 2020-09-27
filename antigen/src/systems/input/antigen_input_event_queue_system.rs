use crate::{core::events::AntigenInputEvent, systems::EventQueueSystem};

pub type AntigenInputEventQueueSystem = EventQueueSystem<AntigenInputEvent>;
