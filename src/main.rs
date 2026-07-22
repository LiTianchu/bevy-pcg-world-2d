use crate::pcg::terrain;
use crate::{
    ascii::plugins::AsciiWorldPlugins, game::plugins::WorldPlugins, game::resources::GameConfig,
};
use bevy::app::{App, ScheduleRunnerPlugin};
use bevy::prelude::*;
use std::env;
use std::time::Duration;

mod ascii;
mod game;
#[allow(dead_code)]
mod helper;
mod pcg;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut seed: u32 = terrain::constants::DEFAULT_TERRAIN_WORLD_SEED;

    if args.contains(&"--seed".to_string()) {
        let seed_index: usize = args
            .iter()
            .position(|p| p == "--seed")
            .expect("Seed value is not provided")
            + 1;

        if seed_index >= args.len() {
            panic!("Seed value is not provided");
        }

        seed = args[seed_index]
            .parse::<u32>()
            .expect("Seed value must be a non-negative number not larger than 4,294,967,295");
    }

    let mut collision_enabled: bool = true;
    if args.contains(&"--no-collision".to_string()) {
        collision_enabled = false;
    }

    let terrain_seed_resource: terrain::resources::TerrainSeed =
        terrain::resources::TerrainSeed(seed);

    let game_config_resource: GameConfig =
        GameConfig::new().with_collision_enabled(collision_enabled);

    if args.contains(&"--ascii".to_string()) {
        App::new()
            .insert_resource(game_config_resource)
            .insert_resource(terrain_seed_resource)
            .add_plugins(MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(
                Duration::from_secs_f64(1.0 / 60.0),
            )))
            .add_plugins(AsciiWorldPlugins)
            .run();
        return;
    } else {
        App::new()
            .insert_resource(game_config_resource)
            .insert_resource(terrain_seed_resource)
            .add_plugins(DefaultPlugins)
            .add_plugins(WorldPlugins)
            .run();
    }
}
