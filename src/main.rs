use crate::{ascii::plugins::AsciiWorldPlugins, game::plugins::WorldPlugins};
use bevy::app::{App, ScheduleRunnerPlugin};
use bevy::prelude::*;
use std::env;
use std::time::Duration;

mod ascii;
mod game;
mod pcg;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.contains(&"--ascii".to_string()) {
        App::new()
            .add_plugins(MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(
                Duration::from_secs_f64(1.0 / 30.0),
            )))
            .add_plugins(AsciiWorldPlugins)
            .run();
        return;
    } else {
        App::new()
            .add_plugins(DefaultPlugins)
            .add_plugins(WorldPlugins)
            .run();
    }
}
