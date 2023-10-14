use pixels::Pixels;

pub const WIDTH: usize = 800;
pub const HEIGHT: usize = 600;

pub struct Color {
    red: u8,
    green: u8,
    blue: u8,
    alpha: u8,
}

#[derive(Clone)]
pub struct Vector {
    pub x: i32,
    pub y: i32,
}

pub struct Line {
    pub start: Vector,
    pub end: Vector,
}

pub struct Circle {
    pub position: Vector,
    pub radius: i32,
}

pub struct Ellipse {
    pub position: Vector,
    pub size: Vector,
}

pub trait Figure {
    fn draw(&self, _screen: &mut [u8]) {}
    fn endpoint(&mut self, _mouse: Vector) {}
}

pub struct Scene {
    pub screen: Pixels,
    pub scene: Vec<Box<dyn Figure>>,
}

impl Line {
    pub fn new(start: Vector, end: Vector) -> Self {
        Line { start, end }
    }
}

impl Figure for Line {
    fn draw(&self, screen: &mut [u8]) {
        let mut current = Vector::new(self.start.x, self.start.y);
        let delta = Vector::new(
            (self.end.x - self.start.x).abs(),
            (self.end.y - self.start.y).abs(),
        );
        let step = Vector::new(
            if self.start.x < self.end.x { 1 } else { -1 },
            if self.start.y < self.end.y { 1 } else { -1 },
        );
        let mut err = delta.x - delta.y;

        loop {
            Scene::set_pixel(&current, Color::new(255, 0, 0, 255), screen);

            if current.x == self.end.x && current.y == self.end.y {
                break;
            }

            let e2 = 2 * err;

            if e2 > -delta.y {
                err -= delta.y;
                current.x += step.x;
            }

            if e2 < delta.x {
                err += delta.x;
                current.y += step.y;
            }
        }
    }

    fn endpoint(&mut self, mouse: Vector) {
        self.end = mouse;
    }
}

impl Circle {
    pub fn new(position: Vector, radius: i32) -> Self {
        Circle { position, radius }
    }
}

impl Figure for Circle {
    fn draw(&self, screen: &mut [u8]) {
        let mut current = Vector::new(0, self.radius);
        let mut delta = 3 - 2 * self.radius;

        while current.x <= current.y {
            Scene::set_pixel(
                &Vector::new(self.position.x + current.x, self.position.y + current.y),
                Color::new(255, 0, 0, 255),
                screen,
            );
            Scene::set_pixel(
                &Vector::new(self.position.x - current.x, self.position.y + current.y),
                Color::new(255, 0, 0, 255),
                screen,
            );
            Scene::set_pixel(
                &Vector::new(self.position.x + current.x, self.position.y - current.y),
                Color::new(255, 0, 0, 255),
                screen,
            );
            Scene::set_pixel(
                &Vector::new(self.position.x - current.x, self.position.y - current.y),
                Color::new(255, 0, 0, 255),
                screen,
            );
            Scene::set_pixel(
                &Vector::new(self.position.x + current.y, self.position.y + current.x),
                Color::new(255, 0, 0, 255),
                screen,
            );
            Scene::set_pixel(
                &Vector::new(self.position.x - current.y, self.position.y + current.x),
                Color::new(255, 0, 0, 255),
                screen,
            );
            Scene::set_pixel(
                &Vector::new(self.position.x + current.y, self.position.y - current.x),
                Color::new(255, 0, 0, 255),
                screen,
            );
            Scene::set_pixel(
                &Vector::new(self.position.x - current.y, self.position.y - current.x),
                Color::new(255, 0, 0, 255),
                screen,
            );

            if delta < 0 {
                delta += 4 * current.x + 6;
            } else {
                delta += 4 * (current.x - current.y) + 10;
                current.y -= 1;
            }

            current.x += 1;
        }
    }

    fn endpoint(&mut self, mouse: Vector) {
        self.radius = f32::sqrt(
            (self.position.x - mouse.x).pow(2) as f32 + (self.position.y - mouse.y).pow(2) as f32,
        ) as i32;
    }
}

impl Ellipse {
    pub fn new(position: Vector, size: Vector) -> Self {
        Ellipse { position, size }
    }
}

impl Figure for Ellipse {
    fn draw(&self, screen: &mut [u8]) {
        let mut current = Vector::new(0, self.size.y);
        let mut delta = Vector::new(
            2 * self.size.y * self.size.y * current.x + self.size.y * self.size.y,
            self.size.y * self.size.y * (current.x + 1) * (current.x + 1)
                + self.size.x * self.size.x * (current.y - 1) * (current.y - 1)
                - self.size.x * self.size.x * self.size.y * self.size.y,
        );

        while delta.x < self.size.x * self.size.x * (2 * current.y - 1) {
            Scene::set_pixel(
                &Vector::new(current.x + self.position.x, current.y + self.position.y),
                Color::new(255, 0, 0, 255),
                screen,
            );
            Scene::set_pixel(
                &Vector::new(-current.x + self.position.x, current.y + self.position.y),
                Color::new(255, 0, 0, 255),
                screen,
            );
            Scene::set_pixel(
                &Vector::new(current.x + self.position.x, -current.y + self.position.y),
                Color::new(255, 0, 0, 255),
                screen,
            );
            Scene::set_pixel(
                &Vector::new(-current.x + self.position.x, -current.y + self.position.y),
                Color::new(255, 0, 0, 255),
                screen,
            );

            if delta.y <= 0 {
                current.x += 1;
                delta.x += 2 * self.size.y * self.size.y;
                delta.y += 2 * self.size.y * self.size.y * (current.x + 1);
            } else {
                current.x += 1;
                current.y -= 1;
                delta.x += 2 * self.size.y * self.size.y;
                delta.y += 2 * self.size.y * self.size.y * (current.x + 1)
                    - 2 * self.size.x * self.size.x * (current.y - 1);
            }
        }

        delta.y = self.size.y
            * self.size.y
            * (current.x as f32 + 0.5) as i32
            * (current.x as f32 + 0.5) as i32
            + self.size.x * self.size.x * (current.y - 1) * (current.y - 1)
            - self.size.x * self.size.x * self.size.y * self.size.y;

        while current.y >= 0 {
            Scene::set_pixel(
                &Vector::new(current.x + self.position.x, current.y + self.position.y),
                Color::new(255, 0, 0, 255),
                screen,
            );
            Scene::set_pixel(
                &Vector::new(-current.x + self.position.x, current.y + self.position.y),
                Color::new(255, 0, 0, 255),
                screen,
            );
            Scene::set_pixel(
                &Vector::new(current.x + self.position.x, -current.y + self.position.y),
                Color::new(255, 0, 0, 255),
                screen,
            );
            Scene::set_pixel(
                &Vector::new(-current.x + self.position.x, -current.y + self.position.y),
                Color::new(255, 0, 0, 255),
                screen,
            );

            if delta.y > 0 {
                current.y -= 1;
                delta.y -= 2 * self.size.x * self.size.x * current.y;
            } else {
                current.y -= 1;
                current.x += 1;
                delta.y += 2 * self.size.y * self.size.y * current.x
                    - 2 * self.size.x * self.size.x * current.y;
            }
        }
    }

    fn endpoint(&mut self, mouse: Vector) {
        self.size = Vector::new(
            (self.position.x - mouse.x).abs(),
            (self.position.y - mouse.y).abs(),
        );
    }
}

impl Vector {
    pub fn new(x: i32, y: i32) -> Self {
        Vector { x, y }
    }
}

impl Color {
    pub fn new(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        Color {
            red,
            green,
            blue,
            alpha,
        }
    }
}

impl Scene {
    pub fn new(screen: Pixels) -> Self {
        Scene {
            screen,
            scene: Vec::new(),
        }
    }

    pub fn push<T: Figure + 'static>(&mut self, figure: T) {
        self.scene.push(Box::new(figure));
    }

    pub fn clear(&mut self) {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                Scene::set_pixel(
                    &Vector::new(x as i32, y as i32),
                    Color::new(0, 0, 0, 255),
                    self.screen.frame_mut(),
                )
            }
        }
    }

    fn set_pixel(position: &Vector, color: Color, screen: &mut [u8]) {
        if position.x < WIDTH as i32
            && position.y < HEIGHT as i32
            && position.x >= 0
            && position.y >= 0
        {
            let index = 4 * (position.y * WIDTH as i32 + position.x) as usize;
            screen[index] = color.red;
            screen[index + 1] = color.green;
            screen[index + 2] = color.blue;
            screen[index + 3] = color.alpha;
        }
    }
}
