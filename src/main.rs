pub mod logic;

use macroquad::prelude::*;
fn window_conf() -> Conf {
    Conf {
        window_title: "Window Conf".to_owned(),
        window_width: 1368,
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

#[macroquad::main(window_conf)]
async fn main() {
    loop {
        clear_background(LIGHTGRAY);
        if is_mouse_button_down(MouseButton::Left) {
            let (mouse_x, mouse_y) = mouse_position();
            draw_circle(mouse_x, mouse_y, 5.0, BLUE);
        }

        draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
        draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);

        draw_text("IT WORKS!", 20.0, 20.0, 30.0, DARKGRAY);

        draw_text("touch the screen!", 20.0, 20.0, 20.0, DARKGRAY);
        println!("hola");
        next_frame().await
    }
}
