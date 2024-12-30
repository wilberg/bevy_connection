use crate::communication::CommunicationPlugin;
use bevy::{prelude::*, remote::BrpRequest, tasks::IoTaskPool};
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

        if let (Some(token), Some(port)) = (token, port) {
            if let Ok(port) = port.parse() {
                app.add_plugins(CommunicationPlugin::default());
                app.insert_resource(Initiator { port, token });
                app.add_systems(Startup, notify_initiator);
            }
        }
    }
}

fn notify_initiator(initiator: Res<Initiator>) {
    let token = initiator.token.clone();
    let port = initiator.port.clone();

    IoTaskPool::get()
        .spawn(async move {
            let url = format!("http://127.0.0.1:{port}");
            let request = BrpRequest {
                jsonrpc: String::from("2.0"),
                method: String::from("connection/notify"),
                id: None,
                params: Some(json! ({ "token": token, "port": port })),
            };

            if let Ok(request_body) = to_string(&request) {
                match Client::new().post(url).body(request_body).send() {
                    Ok(_) => {}
                    Err(error) => {
                        error!("Could not send request: {error}");
                    }
                }
            }
        })
        .detach();
}
