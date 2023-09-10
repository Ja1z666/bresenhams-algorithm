use bresenhams_algorithm::{Circle, Display, Line, Vector};

fn main() {
    let mut display = Display::new();

    display.draw_line(Line::new(Vector::new(0, 0), Vector::new(2, 2)));
    display.draw_circle(Circle::new(Vector::new(16, 16), 11));

    display.print();
}
