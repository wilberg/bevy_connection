use std::process;

use super::event::ConnectionEvent;
use crate::communication::CommunicationPlugin;
use bevy::{
    ecs::system::SystemParam,
    prelude::*,
    remote::{http::HostPort, BrpError, BrpResult},
    tasks::IoTaskPool,
};
use serde::Deserialize;
use serde_json::{from_value, Value};

#[derive(Resource)]
struct Token(pub String);

pub struct InitiatorPlugin;

impl Plugin for InitiatorPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(
            CommunicationPlugin::default().with_method("connection/notify", on_notify_connected),
        );
        app.insert_resource(Token("test".to_string()));
    }
}

#[derive(Deserialize)]
struct NotifyConnectionParams {
    token: String,
    port: u16,
}

fn on_notify_connected(In(params): In<Option<Value>>, world: &mut World) -> BrpResult {
    if let Some(params) = params {
        let params = from_value::<NotifyConnectionParams>(params)
            .map_err(|_| BrpError::internal("Invalid parameters"))?;

        if let Some(token) = world.get_resource::<Token>() {
            if token.0 == params.token {
                world.trigger(ConnectionEvent::Connected(params.port));
                return Ok(Value::Null);
            }
        }
    }

    Err(BrpError::internal("Required parameters were not present."))
}

#[derive(SystemParam)]
pub struct ConnectionManager<'w> {
    port: Res<'w, HostPort>,
    token: Res<'w, Token>,
}

impl ConnectionManager<'_> {
    pub fn connect(&self, path: &str) {
        info!("Will connect to executable at {}.", path);

        let path = String::from(path);
        let port = self.port.0.clone();
        let token = self.token.0.clone();

        IoTaskPool::get()
            .spawn(async move {
                if let Ok(child) = process::Command::new("cargo")
                    .current_dir(path)
                    .arg("run")
                    .arg("--")
                    .arg("--port")
                    .arg(port.to_string())
                    .arg("--token")
                    .arg(token)
                    .spawn()
                {
                    info!("[{}] Connection successful.", child.id());
                }
            })
            .detach();
    }
}
