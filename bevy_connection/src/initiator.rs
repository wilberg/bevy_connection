use std::process;

use crate::{event::MessageEvent, utils::get_available_port};

use super::event::ConnectionEvent;
use bevy::{
    ecs::system::SystemParam,
    prelude::*,
    remote::{
        builtin_methods::BrpListResponse,
        http::{HostPort, RemoteHttpPlugin},
        BrpError, BrpRequest, BrpResponse, BrpResult, RemotePlugin,
    },
    tasks::IoTaskPool,
};
use reqwest::blocking::Client;
use serde::Deserialize;
use serde_json::{from_value, json, to_string, Value};

#[derive(Resource)]
struct Token(pub String);

#[derive(Component)]
struct Connection(pub u16);

pub struct InitiatorPlugin;

impl Plugin for InitiatorPlugin {
    fn build(&self, app: &mut App) {
        if let Some(available_port) = get_available_port() {
            info!("[Initiator] Will listen on port {available_port}");

            app.add_event::<MessageEvent>();
            app.add_event::<ConnectionEvent>();

            app.add_plugins(
                RemotePlugin::default().with_method("connection/notify", on_notify_connected),
            );

            app.add_plugins(RemoteHttpPlugin::default().with_port(available_port));

            app.insert_resource(Token("test".to_string()));
        }
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
                world.spawn(Connection(params.port));
                world.trigger(ConnectionEvent::Connected(params.port));
                return Ok(Value::Null);
            }
        }
    }

    Err(BrpError::internal("Required parameters were not present."))
}

#[derive(Clone)]
pub enum Message {
    RequestComponents(Option<Entity>),
}

impl Message {
    pub fn get_method(&self) -> String {
        match self {
            Message::RequestComponents(_entity) => String::from("bevy/list"),
        }
    }

    pub fn get_params(&self) -> Option<Value> {
        match self {
            Message::RequestComponents(entity) => {
                if let Some(entity) = entity {
                    return Some(json!({
                        "entity": entity
                    }));
                }

                None
            }
        }
    }
}

#[derive(SystemParam)]
pub struct ConnectionManager<'w, 's> {
    port: Res<'w, HostPort>,
    token: Res<'w, Token>,
    connections: Query<'w, 's, &'static Connection>,
}

impl ConnectionManager<'_, '_> {
    pub fn connect(&self, path: &str) {
        info!("[Initiator] Will connect to executable at {}.", path);

        let path = String::from(path);
        let port = self.port.0.clone();
        let token = self.token.0.clone();

        IoTaskPool::get()
            .spawn(async move {
                if let Ok(mut child) = process::Command::new("cargo")
                    .current_dir(path)
                    .arg("run")
                    .arg("--")
                    .arg("--port")
                    .arg(port.to_string())
                    .arg("--token")
                    .arg(token)
                    .spawn()
                {
                    child.wait().unwrap();
                    println!("Disconnected! TODO: Notify through channel.");
                }
            })
            .detach();
    }

    pub fn message(&self, message: Message) {
        for connection in &self.connections {
            let url = format!("http://127.0.0.1:{}", connection.0);
            let message = message.clone();

            IoTaskPool::get()
                .spawn(async move {
                    let request = BrpRequest {
                        jsonrpc: String::from("2.0"),
                        method: message.get_method(),
                        id: None,
                        params: message.get_params(),
                    };

                    if let Ok(request_body) = to_string(&request) {
                        match Client::new().post(url).body(request_body).send() {
                            Ok(result) => {
                                let value = result.json::<Value>().unwrap();
                                let value = value.get("result").unwrap();
                                let list = from_value::<BrpListResponse>(value.clone()).unwrap();

                                println!("{:?}", list);
                            }
                            Err(error) => {
                                error!("[Initiator] Could not send message: {error}");
                            }
                        }
                    }
                })
                .detach();
        }
    }
}
