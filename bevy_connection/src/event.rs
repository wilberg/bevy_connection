use bevy::prelude::*;

#[derive(Event)]
pub enum ConnectionEvent {
    Connected(u16),
    Disconnected(u16),
}

#[derive(Event)]
pub enum MessageEvent {
    Custom(String),
    Ping,
    Pong,
}
