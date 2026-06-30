use bevy::prelude::*;

fn say_hello() {
    println!("Hello Bevy!");
}

fn system_1() {
    println!("Running system 1");
}
fn system_2() {
    println!("Running system 2");
}
fn system_3() {
    println!("Running system 3");
}

fn main() {
    let mut app = App::new();

    app.add_plugins(MinimalPlugins);

    app.add_systems(Update, say_hello);

    app.add_systems(Startup, (system_1, system_2, system_3));

    app.run();
}
