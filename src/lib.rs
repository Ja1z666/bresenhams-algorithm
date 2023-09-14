pub const WHITE: &str = "\x1b[47m";
pub const BLACK: &str = "\x1b[40m";
pub const RESET: &str = "\x1b[0m";

const SIZE: usize = 32;

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

impl Line {
    pub fn new(start: Vector, end: Vector) -> Self {
        Line { start, end }
    }
}

impl Circle {
    pub fn new(position: Vector, radius: i32) -> Self {
        Circle { position, radius }
    }
}

#[derive(Clone, Copy)]
enum Pixel {
    Empty,
    Filled,
}

pub struct Display {
    matrix: [[Pixel; SIZE]; SIZE],
}

impl Vector {
    pub fn new(x: i32, y: i32) -> Self {
        Vector { x, y }
    }
}

impl Display {
    pub fn new() -> Self {
        Display {
            matrix: [[Pixel::Empty; SIZE]; SIZE],
        }
    }

    pub fn draw_line(&mut self, line: Line) {
        let mut current = Vector::new(line.start.x, line.start.y);
        let delta = Vector::new(
            (line.end.x - line.start.x).abs(),
            (line.end.y - line.start.y).abs(),
        );
        let step = Vector::new(
            if line.start.x < line.end.x { 1 } else { -1 },
            if line.start.y < line.end.y { 1 } else { -1 },
        );
        let mut err = delta.x - delta.y;

        loop {
            self.set_pixel(current.x, current.y);

            if current.x == line.end.x && current.y == line.end.y {
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

    pub fn draw_circle(&mut self, circle: Circle) {
        let mut current = Vector::new(0, circle.radius);
        let mut delta = 3 - 2 * circle.radius;

        while current.x <= current.y {
            self.set_pixel(circle.position.x + current.x, circle.position.y + current.y);
            self.set_pixel(circle.position.x - current.x, circle.position.y + current.y);
            self.set_pixel(circle.position.x + current.x, circle.position.y - current.y);
            self.set_pixel(circle.position.x - current.x, circle.position.y - current.y);
            self.set_pixel(circle.position.x + current.y, circle.position.y + current.x);
            self.set_pixel(circle.position.x - current.y, circle.position.y + current.x);
            self.set_pixel(circle.position.x + current.y, circle.position.y - current.x);
            self.set_pixel(circle.position.x - current.y, circle.position.y - current.x);

            if delta < 0 {
                delta += 4 * current.x + 6;
            } else {
                delta += 4 * (current.x - current.y) + 10;
                current.y -= 1;
            }

            current.x += 1;
        }
    }

    fn set_pixel(&mut self, x: i32, y: i32) {
        if x < SIZE as i32 && y < SIZE as i32 && x >= 0 && y >= 0 {
            self.matrix[x as usize][y as usize] = Pixel::Filled;
        }
    }

    pub fn print(&self) {
        for x in 0..SIZE {
            for y in 0..SIZE {
                match self.matrix[y][x] {
                    Pixel::Empty => print!("{}  {}", WHITE, RESET),
                    Pixel::Filled => print!("{}  {}", BLACK, RESET),
                }
            }
            print!("\n");
        }
    }
}
