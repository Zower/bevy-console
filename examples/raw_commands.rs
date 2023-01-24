use bevy::prelude::*;
use bevy_console::{ConsoleCommandEntered, ConsolePlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(ConsolePlugin)
        .add_system(raw_commands)
        .run();
}

fn raw_commands(mut console_commands: EventReader<ConsoleCommandEntered>) {
    for ConsoleCommandEntered { command_name, args } in console_commands.iter() {
        println!(r#"Entered command "{command_name}" with args {:#?}"#, args);
    }
}
