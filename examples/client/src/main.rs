use bevy::prelude::*;
use bevy_connection::prelude::*;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins);
    app.add_plugins(ClientPlugin);

    app.run();
}
