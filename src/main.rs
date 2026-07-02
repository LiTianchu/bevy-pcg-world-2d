use bevy::{prelude::*, reflect::map::Map};
use noise::{NoiseFn, Perlin, Seedable};
use std::fmt;

const DEFAULT_GRID_WIDTH: usize = 64;
const DEFAULT_GRID_HEIGHT: usize = 64;
const TILE_SIZE: f32 = 16.0;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile {
    Wall,
    Floor,
    Void,
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Tile::Wall => write!(f, "W"),
            Tile::Floor => write!(f, "F"),
            Tile::Void => write!(f, "V"),
        }
    }
}

#[derive(Resource)]
struct MapTiles {
    tiles: Vec<Vec<Tile>>,
}

impl MapTiles {
    pub fn new(tiles: Vec<Vec<Tile>>) -> Self {
        Self { tiles }
    }
}

fn generate_map(seed: u32, grid_width: usize, grid_height: usize) -> Vec<Vec<Tile>> {
    let perlin = Perlin::new(seed);
    let mut map = vec![vec![Tile::Void; grid_width]; grid_height];

    let scale = 0.1;
    for y in 0..grid_height {
        for x in 0..grid_width {
            let value = perlin.get([x as f64 * scale, y as f64 * scale]);
            map[y][x] = if value > 0.0 { Tile::Wall } else { Tile::Floor };
        }
    }

    return map;
}

fn draw_map(mut commands: Commands, map: Res<MapTiles>) {
    commands.spawn(Camera2d);
    for y in 0..map.tiles.len() {
        for x in 0..map.tiles[0].len() {
            let tile: Tile = map.tiles[y][x];
            let color: Color = match tile {
                Tile::Wall => Color::srgba(0.5, 0.4, 0.6, 1.0),
                Tile::Floor => Color::srgba(0.2, 0.4, 0.9, 1.0),
                Tile::Void => Color::srgba(0.0, 0.0, 0.0, 1.0),
            };
            commands.spawn((
                Sprite {
                    color: color,
                    custom_size: Some(Vec2::splat(TILE_SIZE)),
                    ..default()
                },
                Transform::from_xyz((x as f32) * TILE_SIZE, (y as f32) * TILE_SIZE, 0.0),
            ));
        }
    }
}

pub struct MapPlugin;
impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        let seed: u32 = 1234;
        app.insert_resource(MapTiles::new(generate_map(
            seed,
            DEFAULT_GRID_WIDTH,
            DEFAULT_GRID_HEIGHT,
        )))
        .add_systems(Startup, draw_map);
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(MapPlugin)
        .run();
}
