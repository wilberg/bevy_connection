use bevy::prelude::*;
use bevy_connection::prelude::*;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins);
    app.add_plugins(InitiatorPlugin);

    app.add_systems(Startup, connect_to_client);
    app.add_observer(on_connection_change);

    app.run();
}

fn connect_to_client(connection: ConnectionManager) {
    connection.connect("./examples/client");
}

fn on_connection_change(trigger: Trigger<ConnectionEvent>, connection: ConnectionManager) {
    if let ConnectionEvent::Connected(port) = trigger.event() {
        info!("[Editor] Connected to client on port {port}.");
        // Request type data.
        connection.message(Message::RequestComponents(None));
    }

    if let ConnectionEvent::Disconnected(port) = trigger.event() {
        info!("[Editor] Disconnected from client on port {port}.");
    }
}
