
/*
definition of all the user events that can happen
this will get past into runtime inputs so the project 
doesnt have to include sdl
return here when unsure what events are available
*/

pub enum Event {
    KeyLeft,
    KeyDown,
    KeyUp,
    KeyRight,
    WheelUp,
    WheelDown,
    MouseLeft,
    MouseRight,
    Escape,
    None,
}