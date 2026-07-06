use bevy::prelude::*;
use noise::{NoiseFn, Perlin};
use std::collections::hash_set::HashSet;
use std::fmt;

const DEFAULT_GRID_WIDTH: usize = 64;
const DEFAULT_GRID_HEIGHT: usize = 64;
const TILE_SIZE: f32 = 16.0;
const TILE_DIMESNION: Vec2 = Vec2::splat(TILE_SIZE);
const DEFAULT_GRID_CENTER: Vec2 = Vec2 {
    x: (DEFAULT_GRID_WIDTH as f32 * TILE_SIZE) / 2.0,
    y: (DEFAULT_GRID_HEIGHT as f32 * TILE_SIZE) / 2.0,
};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Tile {
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
    pub fn new() -> Self {
        Self {
            tiles: vec![vec![]],
        }
    }

    pub fn with_tiles(mut self, tiles: Vec<Vec<Tile>>) -> Self {
        self.tiles = tiles;
        self
    }

    pub fn set_tile(&mut self, x: usize, y: usize, tile: Tile) -> Result {
        let curr_tile: Tile = self.tile(x, y)?;
        if curr_tile != tile {
            self.tiles[y][x] = tile;
        }
        Ok(())
    }

    pub fn dimension(&self) -> UVec2 {
        if self.tiles.len() == 0 {
            UVec2 { x: 0, y: 0 }
        } else {
            UVec2 {
                x: self.tiles[0].len() as u32,
                y: self.tiles.len() as u32,
            }
        }
    }

    pub fn tiles_iter(&self) -> impl Iterator<Item = (UVec2, Tile)> + '_ {
        self.tiles.iter().enumerate().flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(move |(x, &tile)| (UVec2::new(x as u32, y as u32), tile))
        })
    }

    pub fn tile(&self, x: usize, y: usize) -> Result<Tile> {
        if self.tiles.len() == 0 {
            return Err("Grid is empty".into());
        }
        if y < self.tiles.len() && x < self.tiles[0].len() {
            Ok(self.tiles[y][x])
        } else {
            Err("Tile coordinate out of boundary".into())
        }
    }

    pub fn tiles_of_type(&self, tile_type: Tile) -> HashSet<UVec2> {
        let dim: UVec2 = self.dimension();
        let mut found_set: HashSet<UVec2> = HashSet::new();
        for y in 0..dim.y {
            for x in 0..dim.x {
                if self.tiles[y as usize][x as usize] == tile_type {
                    found_set.insert(UVec2 { x: x, y: y });
                }
            }
        }
        return found_set;
    }
}

#[derive(Component)]
struct ObjectOnGrid {
    internal_translation: Vec3,
}

impl ObjectOnGrid {
    pub fn new() -> Self {
        Self {
            internal_translation: Vec3::ZERO,
        }
    }

    pub fn with_internal_translation(mut self, translation: Vec3) -> Self {
        self.internal_translation = translation;
        self
    }
}

#[derive(Component)]
struct Movable {
    speed: f32,
}

impl Movable {
    pub fn new() -> Self {
        Self { speed: 1.0 }
    }

    pub fn with_speed(mut self, new_speed: f32) -> Self {
        self.speed = new_speed;
        self
    }
}

#[derive(Component)]
struct Player {
    name: String,
}

impl Player {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }
}

fn generate_map(seed: u32, grid_width: usize, grid_height: usize) -> MapTiles {
    let perlin = Perlin::new(seed);
    let mut map = vec![vec![Tile::Void; grid_width]; grid_height];

    let scale = 0.1;
    for y in 0..grid_height {
        for x in 0..grid_width {
            let value = perlin.get([x as f64 * scale, y as f64 * scale]);
            map[y][x] = if value > 0.0 { Tile::Wall } else { Tile::Floor };
        }
    }

    return MapTiles::new().with_tiles(map);
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Projection::from(OrthographicProjection::default_2d()),
        Transform::from_xyz(DEFAULT_GRID_CENTER.x, DEFAULT_GRID_CENTER.y, 100.0),
    ));
}

pub fn grid_to_pos(x: usize, y: usize) -> Vec3 {
    return Vec3 {
        x: x as f32 * TILE_SIZE,
        y: y as f32 * TILE_SIZE,
        z: 0.0,
    };
}

pub fn pos_to_grid(pos: Vec3) -> Vec3 {
    let x: f32 = (pos.x / TILE_SIZE).round();
    let y: f32 = (pos.y / TILE_SIZE).round();
    return Vec3 { x, y, z: pos.z };
}

fn spawn_player(mut commands: Commands, map: Res<MapTiles>) {
    let player: Player = Player::new("Player");
    let tiles: HashSet<UVec2> = map.tiles_of_type(Tile::Floor);

    let default_spawn_place: UVec2 = UVec2 { x: 0, y: 0 };
    let spawn_place: UVec2 = tiles.iter().next().copied().unwrap_or(default_spawn_place);

    println!(
        "Player spawned: {}, Position: {}",
        player.name(),
        spawn_place
    );

    let spawn_translation: Vec3 =
        grid_to_pos(spawn_place.x as usize, spawn_place.y as usize).with_z(1.0);

    commands.spawn((
        player,
        Transform::from_translation(spawn_translation),
        Sprite {
            color: Color::srgba(0.8, 0.2, 0.1, 1.0),
            custom_size: Some(TILE_DIMESNION),
            ..default()
        },
        Movable::new().with_speed(2.0),
        ObjectOnGrid::new().with_internal_translation(spawn_translation),
    ));
}

pub fn tile_appearance(tile: Tile) -> Color {
    let color: Color = match tile {
        Tile::Wall => Color::srgba(0.5, 0.4, 0.6, 1.0),
        Tile::Floor => Color::srgba(0.2, 0.4, 0.9, 1.0),
        Tile::Void => Color::srgba(0.0, 0.0, 0.0, 1.0),
    };
    return color;
}

fn draw_map(mut commands: Commands, map: Res<MapTiles>) {
    for (coord, tile) in map.tiles_iter() {
        commands.spawn((
            Sprite {
                color: tile_appearance(tile),
                custom_size: Some(TILE_DIMESNION),
                ..default()
            },
            Transform::from_translation(grid_to_pos(coord.x as usize, coord.y as usize)),
        ));
    }
}

fn handle_player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    query: Query<(&mut Transform, &Player, &Movable, &mut ObjectOnGrid)>,
) {
    let mut direction: Vec2 = Vec2::ZERO;
    if keyboard_input.pressed(KeyCode::KeyW) {
        direction.y = 1.0;
    }

    if keyboard_input.pressed(KeyCode::KeyS) {
        direction.y = -1.0;
    }

    if keyboard_input.pressed(KeyCode::KeyA) {
        direction.x = -1.0;
    }

    if keyboard_input.pressed(KeyCode::KeyD) {
        direction.x = 1.0;
    }

    direction = direction.normalize_or_zero();

    for (mut transform, _player, movable, mut object_on_grid) in query {
        object_on_grid.internal_translation.x += direction.x * movable.speed;
        object_on_grid.internal_translation.y += direction.y * movable.speed;
        transform.translation = pos_to_grid(object_on_grid.internal_translation);
        println!("Player movement direction: {:?}", direction);
        println!("Player moved to position: {:?}", transform.translation);
    }
}

pub struct WorldPlugin;
impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        let seed: u32 = 1234;
        app.insert_resource(generate_map(seed, DEFAULT_GRID_WIDTH, DEFAULT_GRID_HEIGHT))
            .add_systems(Startup, (setup_camera, draw_map, spawn_player).chain())
            .add_systems(Update, handle_player_movement);
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(WorldPlugin)
        .run();
}
