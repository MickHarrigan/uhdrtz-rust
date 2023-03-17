use bevy::prelude::*;
// this is going to be where the states will be initialized and where they will control the outcome of the system
fn main() {}

// States that the system can be in
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Default, States)]
pub enum RunningStates {
    #[default]
    Setup,
    Running,
    Loading,
    Standby, // this one is more for the fact that there may be some state that I have no idea of that is just sitting and doing nothing
}
