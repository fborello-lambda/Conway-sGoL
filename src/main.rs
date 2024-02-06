pub mod logic;

use logic::Grid;
use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Window Conf".to_owned(),
        window_width: 800,
        window_height: 600,
        ..Default::default()
    }
}
const SQUARES_X: usize = 50;
const SQUARES_Y: usize = 25;

#[macroquad::main(window_conf)]
async fn main() {
    let mut running: bool = false;
    let mut game = Grid::new(SQUARES_X, SQUARES_Y);
    let mut sq_size;
    let mut now = get_time();
    let mut clicked: bool = false;
    let mut click = get_time();

    loop {
        clear_background(LIGHTGRAY);

        if is_key_pressed(KeyCode::Space) || is_key_pressed(KeyCode::S) {
            running = !running;
        }
        if is_key_pressed(KeyCode::R) {
            // reset grid state
            running = false;
            clicked = false;
            game = Grid::new(SQUARES_X, SQUARES_Y);
        }

        if running {
            draw_text("Running...", 10., 100., 128.0, RED);
        } else {
            draw_text("Not Running...", 10., 100., 128.0, RED);
        }
        draw_text("S or Space to Start/Stop", 1000., 100., 128.0, BLACK);
        draw_text("R to Reset", 2800., 100., 128.0, BLACK);

        //Window settings

        let a = (screen_width() - 20.) / SQUARES_X as f32;
        let b = (screen_height() - 60.) / SQUARES_Y as f32;
        sq_size = a.min(b);

        let offset_x = (screen_width() - 20. - sq_size * SQUARES_X as f32) / 2. + 10.;
        let offset_y_up = (screen_height() - 60. - sq_size * SQUARES_Y as f32) / 2. + 50.;
        let offset_y_down = (screen_height() - 60. - sq_size * SQUARES_Y as f32) / 2. + 10.;

        //Draw background
        draw_rectangle(
            offset_x,
            offset_y_up,
            screen_width() - offset_x * 2.,
            screen_height() - offset_y_down - offset_y_up,
            WHITE,
        );

        for x in 0..game.width {
            for y in 0..game.height {
                draw_rectangle(
                    offset_x + (x as f32) * sq_size,
                    offset_y_up + (y as f32) * sq_size,
                    sq_size,
                    sq_size,
                    {
                        if game.grid[y][x].alive {
                            DARKGREEN
                        } else {
                            WHITE
                        }
                    },
                );
            }
        }

        for i in 1..SQUARES_Y {
            draw_line(
                offset_x,
                offset_y_up + sq_size * i as f32,
                screen_width() - offset_x,
                offset_y_up + sq_size * i as f32,
                2.,
                LIGHTGRAY,
            );
        }
        for i in 1..SQUARES_X {
            draw_line(
                offset_x + sq_size * i as f32,
                offset_y_up,
                offset_x + sq_size * i as f32,
                screen_height() - offset_y_down,
                2.,
                LIGHTGRAY,
            );
        }

        if is_mouse_button_down(MouseButton::Left) && !clicked {
            let (mouse_x, mouse_y) = mouse_position();
            if mouse_x > offset_x && mouse_y > offset_y_up {
                let y = ((mouse_y - offset_y_up) / sq_size).floor() as usize;
                let x = ((mouse_x - offset_x) / sq_size).floor() as usize;
                game.change_cell(y, x);
            }
            clicked = true;
            click = get_time();
        }
        if clicked && (get_time() - click > 0.15) {
            clicked = false;
        }

        if running && (get_time() - now > 0.5) {
            game.update();
            now = get_time();
        }

        next_frame().await
    }
}
