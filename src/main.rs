use bevy::prelude::*;

fn say_hello() {
    println!("Hello Bevy!");
}

fn main() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app.run();
}
