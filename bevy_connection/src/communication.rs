use bevy::{
    prelude::*,
    remote::{http::RemoteHttpPlugin, BrpResult, RemotePlugin},
};
use serde_json::Value;

/// An enum of events which will be fired when a message has been received.
///
/// ## Example
/// ```rust
/// use bevy::prelude::*;
/// use bevy_communication::prelude::*;
///
/// fn send_message(mut writer: EventWriter<MessageEvent>) {
///    writer.send(MessageEvent::Custom("Hello, world!".to_string()));
/// }
///
/// fn message_event_listener(mut reader: EventReader<MessageEvent>) {
///     for event in reader {
///         match event {
///             MessageEvent::Custom(message) => {
///                 println!("Received message: {}", message);
///             }
///             MessageEvent::Ping => {
///                 println!("Received ping!");
///             }
///             MessageEvent::Pong => {
///                 println!("Received pong!");
///             }
///         }
///     }
/// }
/// ```
///
#[derive(Event)]
pub enum MessageEvent {
    Custom(String),
    Ping,
    Pong,
}

pub struct CommunicationPlugin {
    port: u16,
    remote_plugin: RemotePlugin,
}

impl CommunicationPlugin {
    pub fn with_port(port: u16) -> CommunicationPlugin {
        CommunicationPlugin {
            port,
            remote_plugin: RemotePlugin::default(),
        }
    }

    pub fn with_method<M>(
        mut self,
        name: impl Into<String>,
        handler: impl IntoSystem<In<Option<Value>>, BrpResult, M>,
    ) -> Self {
        self.remote_plugin = self.remote_plugin.with_method(name, handler);
        self
    }
}

impl Default for CommunicationPlugin {
    fn default() -> Self {
        CommunicationPlugin {
            port: 1524,
            remote_plugin: RemotePlugin::default(),
        }
    }
}

impl Plugin for CommunicationPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<MessageEvent>();
        app.add_plugins(RemoteHttpPlugin::default().with_port(self.port));
    }
}
