use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Space Shooter".into(),
        icon: None,
        window_width: 1280,
        window_height: 720,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut x = screen_width() / 2.0;
    let mut y = screen_height() / 2.0;

    loop {
        clear_background(BLACK);

        if is_key_down(KeyCode::Right) {
            x += 300.0 * get_frame_time();
        }
        if is_key_down(KeyCode::Left) {
            x -= 300.0 * get_frame_time();
        }
        if is_key_down(KeyCode::Down) {
            y += 300.0 * get_frame_time();
        }
        if is_key_down(KeyCode::Up) {
            y -= 300.0 * get_frame_time();
        }

        draw_circle(x, y, 15.0, WHITE);
        draw_text("move the ball with arrow keys", 20.0, 20.0, 20.0, WHITE);
        next_frame().await
    }
}
