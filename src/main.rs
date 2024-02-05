pub mod logic;

use macroquad::prelude::*;
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

const SQUARES: i16 = 32;

#[macroquad::main(window_conf)]
async fn main() {
    loop {
        clear_background(LIGHTGRAY);

        let offset_x = 20.;
        let offset_y = 50.;
        let sq_size = (screen_width() - offset_x * 2.) / SQUARES as f32;

        draw_rectangle(
            offset_x,
            offset_y,
            screen_width() - 20.,
            screen_height() - 20.,
            WHITE,
        );

        for i in 1..SQUARES {
            draw_line(
                offset_x,
                offset_y + sq_size * i as f32,
                screen_width() - offset_x,
                offset_y + sq_size * i as f32,
                2.,
                LIGHTGRAY,
            );
            draw_line(
                offset_x + sq_size * i as f32,
                offset_y,
                offset_x + sq_size * i as f32,
                screen_height() - offset_y,
                2.,
                LIGHTGRAY,
            );
        }

        draw_rectangle(
            offset_x + 0. * sq_size,
            offset_y + 0. * sq_size,
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
