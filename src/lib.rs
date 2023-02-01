pub use opencv;
pub mod components;
pub mod plugin;
mod systems;

pub mod prelude {
    pub use crate::{components::CaptureDevice, plugin::ZoetropePlugin};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
