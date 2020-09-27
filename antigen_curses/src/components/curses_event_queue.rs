use antigen::components::EventQueue;

pub type CursesEvent = pancurses::Input;
pub type CursesEventQueue = EventQueue<CursesEvent>;
