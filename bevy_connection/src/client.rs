use crate::{
    event::{ConnectionEvent, MessageEvent},
    utils::get_available_port,
};
use bevy::{
    prelude::*,
    remote::{
        http::{HostPort, RemoteHttpPlugin},
        BrpRequest, RemotePlugin,
    },
    tasks::IoTaskPool,
};
use clap::Parser;
use reqwest::blocking::Client;
use serde_json::{json, to_string};

#[derive(Parser)]
struct ClientArguments {
    #[arg(short, long)]
    token: Option<String>,

    #[arg(short, long)]
    port: Option<String>,
}

#[derive(Resource)]
struct Initiator {
    port: u16,
    token: String,
}

/// ```rust
/// fn main() {
///     let mut app = App::new();
///
///     #[cfg(debug_assertions)]
///     app.add_plugins(ClientPlugin);
///
///     app.run();
/// }
/// ```
pub struct ClientPlugin;

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
        let ClientArguments { token, port } = ClientArguments::parse();

        if let (Some(initiator_token), Some(initiator_port)) = (token, port) {
            if let Ok(initiator_port) = initiator_port.parse() {
                if let Some(available_port) = get_available_port() {
                    info!("[Client] Will listen on port {available_port}");

                    app.add_event::<MessageEvent>();
                    app.add_event::<ConnectionEvent>();

                    app.add_plugins(RemotePlugin::default());
                    app.add_plugins(RemoteHttpPlugin::default().with_port(available_port));

                    app.insert_resource(Initiator {
                        port: initiator_port,
                        token: initiator_token,
                    });
                    app.add_systems(Startup, notify_initiator);
                }
            }
        }
    }
}

fn notify_initiator(initiator: Res<Initiator>, client_port: Res<HostPort>) {
    let token = initiator.token.clone();
    let port = initiator.port.clone();

    let client_port = client_port.0.clone();

    IoTaskPool::get()
        .spawn(async move {
            let url = format!("http://127.0.0.1:{port}");
            let request = BrpRequest {
                jsonrpc: String::from("2.0"),
                method: String::from("connection/notify"),
                id: None,
                params: Some(json! ({ "token": token, "port": client_port })),
            };

            if let Ok(request_body) = to_string(&request) {
                match Client::new().post(url).body(request_body).send() {
                    Ok(_) => {}
                    Err(error) => {
                        error!("[Client] Could not send request: {error}");
                    }
                }
            }
        })
        .detach();
}
