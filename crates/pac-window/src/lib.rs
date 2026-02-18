//! Window management for PAC game engine

pub mod window;

pub use window::{create_event_loop, GameWindow};
pub use winit;

pub mod time;
pub use time::{DeltaTime, FixedTimestep, FpsCounter};

pub mod input;
pub use input::{InputMap, InputState};
