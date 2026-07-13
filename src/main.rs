use crate::game::plugins::WorldPlugin;
use bevy::app::{App, ScheduleRunnerPlugin};
use bevy::prelude::*;
use std::env;
use std::time::Duration;

mod ascii;
mod game;
mod pcg;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.contains(&"--window".to_string()) {
        App::new()
            .add_plugins(DefaultPlugins)
            .add_plugins(WorldPlugin)
            .run();
    } else {
        App::new()
            .add_plugins(MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(
                Duration::from_secs_f64(1.0 / 30.0),
            )))
            .add_plugins(ascii::plugins::AsciiWorldPlugin)
            .run();
        return;
    }
}
