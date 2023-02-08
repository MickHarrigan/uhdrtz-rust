pub mod components;
pub mod plugin;
pub use nokhwa;
mod systems;

pub mod prelude {
    pub use crate::{components::CaptureDevice, plugin::ZoetropePlugin}; // temporary names for right now, these may change based on the current needs of the project
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // let result = add(2, 2);
        // assert_eq!(result, 4);
    }
}
