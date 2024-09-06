use raylib::prelude::*;

enum Size {
    SMALL, MEDIUM, LARGE
}
struct Circle {
    position: Vector2,
    size: Size,
    is_out_of_view: bool
}
/*
impl Circle {

    fn new() -> Circle {
        return Circle {
            position: Vector2 {
                x: 12.0,
                y: 12.0
            },
            size: Size::MEDIUM,
            is_out_of_view: false
        };
    }

    fn update(&mut self) {
        let speed = match self.size {
            Size::SMALL  => 2,
            Size::MEDIUM => 4,
            Size::LARGE  => 8
        };
    }
}
*/
fn main() {
    let (mut rl, thread) = raylib::init()
        .size(640, 480)
        .title("Hello, World")
        .build();

//    let particles: Vec<Circle> = vec!();

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::WHITE);
        d.draw_text("Hello, world!", 12, 12, 20, Color::BLACK);
    }
}