use raylib::prelude::*;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(800, 600)
        .title("Basic Raylib Application")
        .build();

    rl.set_target_fps(60);

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::RAYWHITE);
        d.draw_text("Hello, Raylib!", 12, 12, 20, Color::DARKGRAY);
        d.draw_fps(10, 10);
    }
}
