use raylib::prelude::*;

#[derive(Clone)]
enum Size {
    SMALL, MEDIUM, LARGE
}

#[derive(Clone)]
struct Circle {
    position: Vector2,
    size: Size,
    is_out_of_view: bool
}

impl Circle {

    fn new(rl: &RaylibHandle, is_random_height: bool) -> Circle {
        let random_horizontal: i32 = rl.get_random_value(0..rl.get_screen_width()-1);
        let vertical: i32 = match is_random_height {
            true  => rl.get_random_value(0..rl.get_screen_height()-1),
            false => 0
        };
        return Circle {
            position: Vector2 {
                x: (random_horizontal + 16) as f32 - 8.0,
                y: vertical as f32 - 8.0
            },
            size: match rl.get_random_value(0..2) {
                0 => Size::SMALL,
                1 => Size::MEDIUM,
                _ => Size::LARGE
            },
            is_out_of_view: false
        };
    }

    fn update(&mut self, rl: &RaylibHandle) {
        self.position.y = self.position.y + ((rl.get_screen_height() / 60 / 4) as f32 * match self.size {
            Size::SMALL  => 0.25,
            Size::MEDIUM => 0.5,
            Size::LARGE  => 1.0
        });

        if self.position.y >= rl.get_screen_height() as f32 + 8.0 {
            self.is_out_of_view = true;
        }

    }

}

struct Starfield {
    stars: Vec<Circle>
}

impl Starfield {

    fn new(rl: &RaylibHandle) -> Starfield {
        let mut starfield: Starfield = Starfield { stars: vec!() };
        for _ in 1..50 {
            starfield.stars.push(Circle::new(rl, true));
        }
        return starfield;
    }

    fn update(&mut self, rl: &RaylibHandle) {

        let mut new_starfield: Vec<Circle> = vec!();

        let random: i32 = rl.get_random_value(1..7);
        if random == 3 {
            let new_star: Circle = Circle::new(rl, false);
            new_starfield.push(new_star);
        }

        loop {
            match self.stars.pop() {
                Some(mut star) => {
                    star.update(&rl);
                    if !star.is_out_of_view {
                        new_starfield.push(star);
                    } else {
                        println!("Removing star at {}, {}", star.position.x, star.position.y);
                    }
                },
                None => break
            }
        }

        self.stars = new_starfield;
    }

    fn draw(&mut self, d: &mut RaylibDrawHandle<'_>) {
        for star in self.stars.clone() {
            d.draw_circle(
                star.position.x as i32,
                star.position.y as i32,
                match star.size {
                    Size::SMALL  => 1.0,
                    Size::MEDIUM => 1.5,
                    Size::LARGE  => 2.0
                },
                Color::WHITE);
        }
    }

}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(800, 600)
        .title("Space Arcade")
        .build();

    let mut starfield: Starfield = Starfield::new(&rl);

    rl.set_target_fps(60);
    //rl.set_window_max_size(800, 600);
    while !rl.window_should_close() {

        starfield.update(&rl);

        let mut d = rl.begin_drawing(&thread);

        starfield.draw(&mut d);

        d.clear_background(Color::BLACK);
    }
}