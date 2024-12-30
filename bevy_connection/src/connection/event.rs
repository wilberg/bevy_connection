use bevy::prelude::*;

#[derive(Event)]
pub enum ConnectionEvent {
    Connected(u16),
    Disconnected(u16),
}
