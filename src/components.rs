use bevy::prelude::*;
use opencv::videoio::*;
use std::{
    ptr,
    sync::{Arc, Mutex},
    thread::{self},
};

#[derive(Component)]
pub struct CaptureDevice {
    pub dev: Mutex<VideoCapture>,
}
