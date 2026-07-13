use crate::{
    ascii::resources::TerminalInput,
    game::{
        components::{Movable, ObjectOnGrid},
        constants,
        player::components::Player,
        utils::try_move,
    },
    pcg::terrain::{self, tile},
};
use bevy::prelude::*;
use crossterm::{
    ExecutableCommand, QueueableCommand, cursor, event,
    event::KeyCode as CrosstermKeyCode,
    execute,
    style::{self, Color as CrosstermColor, Stylize},
    terminal,
};
use std::io::{Write, stdout};
use std::time::Duration;

pub fn read_terminal_input(mut terminal_input: ResMut<TerminalInput>) {
    terminal_input.pressed_key = None;

    let Ok(has_event) = event::poll(Duration::from_millis(0)) else {
        return;
    };

    if !has_event {
        return;
    }

    let Ok(event) = event::read() else {
        return;
    };

    if let event::Event::Key(key_event) = event {
        terminal_input.pressed_key = Some(key_event.code);
    }
}

pub fn setup_terminal() -> Result<()> {
    let mut out = stdout();
    execute!(out, cursor::Hide)?;
    let _ = terminal::enable_raw_mode();
    out.execute(terminal::Clear(terminal::ClearType::All))?;
    Ok(())
}

#[allow(dead_code)]
pub fn cleanup_terminal() {
    let _ = terminal::disable_raw_mode();
}

pub fn render_ascii(
    terrain: Res<terrain::resources::TerrainWorld>,
    player_query: Query<
        &Transform,
        (
            With<ObjectOnGrid>,
            With<Player>,
            With<Movable>,
            Changed<Transform>,
        ),
    >,
) -> Result<()> {
    let mut so = stdout();

    if let Ok(player_transform) = player_query.single() {
        let (chunk_coord, local_tile_coord) =
            terrain::utils::pos_to_cell_world(player_transform.translation, &terrain);

        let world_ivec2: IVec2 = terrain::utils::get_world_ivec2(chunk_coord, local_tile_coord);

        // compute ascii camera rect
        let camera_width: u32 =
            constants::ASCII_CAMERA_SIZE * constants::ASCII_CAMERA_ASPECT_RATIO.x;
        let camera_height: u32 =
            constants::ASCII_CAMERA_SIZE * constants::ASCII_CAMERA_ASPECT_RATIO.y;
        let camera_extent: UVec2 = UVec2 {
            x: camera_width / 2,
            y: camera_height / 2,
        };

        let Ok(_cmd) = so.queue(cursor::MoveTo(0, 0)) else {
            return Err("Failed to move cursor to (0, 0)".into());
        };

        let lower_left: IVec2 = IVec2 {
            x: world_ivec2.x - camera_extent.x as i32,
            y: world_ivec2.y - camera_extent.y as i32,
        };

        let upper_right: IVec2 = IVec2 {
            x: world_ivec2.x + camera_extent.x as i32,
            y: world_ivec2.y + camera_extent.y as i32,
        };

        for y in (lower_left.y)..=(upper_right.y) {
            for x in (lower_left.x)..=(upper_right.x) {
                let tile_type: tile::Tile = terrain
                    .tile_at_world_ivec2(IVec2 { x, y })
                    .unwrap_or(tile::Tile::Void);
                let tile_char: char = tile::tile_appearance_ascii(tile_type);
                let color: Srgba = tile::tile_color(tile_type).to_srgba();

                let local_x = (x - lower_left.x) as u16;
                let local_y = (upper_right.y - y) as u16; // invert y for terminal coordinates
                so.queue(cursor::MoveTo(local_x, local_y))?;
                so.queue(style::PrintStyledContent(tile_char.with(
                    CrosstermColor::Rgb {
                        r: (color.red.clamp(0.0, 1.0) * 255.0) as u8,
                        g: (color.green.clamp(0.0, 1.0) * 255.0) as u8,
                        b: (color.blue.clamp(0.0, 1.0) * 255.0) as u8,
                    },
                )))?;
            }
        }
        so.flush()?;
    };
    Ok(())
}

pub fn handle_terminal_player_movement(
    terrain: Res<terrain::resources::TerrainWorld>,
    time: Res<Time>,
    terminal_input: Res<TerminalInput>,
    mut query: Query<
        (&mut Transform, &mut Movable, &mut ObjectOnGrid),
        (With<Player>, With<Movable>, With<ObjectOnGrid>),
    >,
) {
    let mut direction = Vec2::ZERO;

    match terminal_input.pressed_key {
        Some(CrosstermKeyCode::Char('w')) | Some(CrosstermKeyCode::Up) => {
            direction.y = 1.0;
        }
        Some(CrosstermKeyCode::Char('s')) | Some(CrosstermKeyCode::Down) => {
            direction.y = -1.0;
        }
        Some(CrosstermKeyCode::Char('a')) | Some(CrosstermKeyCode::Left) => {
            direction.x = -1.0;
        }
        Some(CrosstermKeyCode::Char('d')) | Some(CrosstermKeyCode::Right) => {
            direction.x = 1.0;
        }
        _ => return,
    }

    direction = direction.normalize_or_zero();

    if direction == Vec2::ZERO {
        return;
    }

    for (mut transform, mut movable, mut object_on_grid) in &mut query {
        movable.last_step_time = None;

        try_move(
            &mut transform,
            &mut movable,
            &mut object_on_grid,
            direction,
            &time,
            &terrain,
        );
    }
}
