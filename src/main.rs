pub mod logic;

use logic::Grid;
use macroquad::prelude::*;
use macroquad::ui;

fn window_conf() -> Conf {
    Conf {
        window_title: "Window Conf".to_owned(),
        window_width: 768,
        window_height: 768,
        fullscreen: false,
        #[cfg(feature = "metal")]
        platform: miniquad::conf::Platform {
            apple_gfx_api: miniquad::conf::AppleGfxApi::Metal,
            ..Default::default()
        },
        ..Default::default()
    }
}

const SQUARES_X: usize = 100;
const SQUARES_Y: usize = 50;

#[macroquad::main(window_conf)]
async fn main() {
    let mut running: bool = false;
    let mut game = Grid::new(SQUARES_X, SQUARES_Y);
    let mut sq_size;
    loop {
        clear_background(LIGHTGRAY);

        if ui::root_ui().button(vec2(screen_width() / 2.0 - 35., 10.), "START") {
            running = !running;
        }
        if ui::root_ui().button(vec2(screen_width() / 2.0 - 35., 40.), "RESET") {
            // reset grid state
            todo!()
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

        //Draw grid

        draw_rectangle(
            offset_x + 0. * sq_size,
            offset_y_up + 0. * sq_size,
            sq_size,
            sq_size,
            DARKGREEN,
        );

        if is_mouse_button_down(MouseButton::Left) {
            let (mouse_x, mouse_y) = mouse_position();
            draw_circle(mouse_x, mouse_y, 5.0, BLUE);
        }

        next_frame().await
    }
}
