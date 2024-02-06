pub mod logic;

use logic::Grid;
use macroquad::prelude::*;
use macroquad::ui;
use std::time::Instant;

fn window_conf() -> Conf {
    Conf {
        window_title: "Window Conf".to_owned(),
        window_width: 600,
        window_height: 600,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut running: bool = false;
    let mut SQUARES_X: usize = 50;
    let mut SQUARES_Y: usize = 25;
    let mut game = Grid::new(SQUARES_X, SQUARES_Y);
    let mut sq_size;
    let mut now = Instant::now();
    let mut clicked: bool = false;
    let mut click = Instant::now();

    loop {
        clear_background(LIGHTGRAY);

        /*if is_key_pressed(KeyCode::Up) {
            SQUARES_Y += 1;
        }
        if is_key_pressed(KeyCode::Down) {
            SQUARES_Y -= 1;
        }
        if is_key_pressed(KeyCode::Right) {
            SQUARES_X += 1;
        }
        if is_key_pressed(KeyCode::Left) {
            SQUARES_X -= 1;
        }
        */
        if ui::root_ui().button(vec2(screen_width() / 2.0 - 35., 10.), "START")
            || is_key_pressed(KeyCode::Space)
            || is_key_pressed(KeyCode::S)
        {
            running = !running;
        }
        if ui::root_ui().button(vec2(screen_width() / 2.0 - 35., 30.), "RESET")
            || is_key_pressed(KeyCode::R)
        {
            // reset grid state
            running = false;
            clicked = false;
            game = Grid::new(SQUARES_X, SQUARES_Y);
        }

        if running {
            draw_text("Runnning...", 10., 25., 30.0, RED);
        } else {
            draw_text("Not Runnning...", 10., 25., 30.0, RED);
        }

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
            click = Instant::now();
        }
        if clicked && (click.elapsed().as_millis() > 150) {
            clicked = false;
        }

        if running && now.elapsed().as_millis() > 500 {
            game.update();
            now = Instant::now();
        }

        next_frame().await
    }
}
