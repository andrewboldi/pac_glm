//! Window management for PAC game engine

pub use winit;

pub mod time;
pub use time::{DeltaTime, FixedTimestep, FpsCounter};
