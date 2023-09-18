use bresenhams_algorithm::{Circle, Display, Ellipse, Line, Vector};
use dialoguer::{console::Term, theme::ColorfulTheme, Input, Select};

fn main() {
    let mut display = Display::new();

    loop {
        let items = ["Line", "Circle", "Ellipse", "Exit"];
        let selection = Select::with_theme(&ColorfulTheme::default())
            .items(&items)
            .default(0)
            .interact_on_opt(&Term::stderr())
            .unwrap();

        match selection {
            Some(index) => match index {
                0 => {
                    let mut line = Line::new(Vector::new(0, 0), Vector::new(0, 0));

                    line.start.x = Input::with_theme(&ColorfulTheme::default())
                        .with_prompt("Line start x: ")
                        .default("0".to_string())
                        .interact_text()
                        .unwrap()
                        .parse()
                        .unwrap();

                    line.start.y = Input::with_theme(&ColorfulTheme::default())
                        .with_prompt("Line start y: ")
                        .default("0".to_string())
                        .interact_text()
                        .unwrap()
                        .parse()
                        .unwrap();

                    line.end.x = Input::with_theme(&ColorfulTheme::default())
                        .with_prompt("Line end x: ")
                        .default("0".to_string())
                        .interact_text()
                        .unwrap()
                        .parse()
                        .unwrap();

                    line.end.y = Input::with_theme(&ColorfulTheme::default())
                        .with_prompt("Line end y: ")
                        .default("0".to_string())
                        .interact_text()
                        .unwrap()
                        .parse()
                        .unwrap();

                    display.draw_line(line);
                }
                1 => {
                    let mut circle = Circle::new(Vector::new(0, 0), 0);

                    circle.position.x = Input::with_theme(&ColorfulTheme::default())
                        .with_prompt("Circle position x: ")
                        .default("0".to_string())
                        .interact_text()
                        .unwrap()
                        .parse()
                        .unwrap();

                    circle.position.y = Input::with_theme(&ColorfulTheme::default())
                        .with_prompt("Circle position y: ")
                        .default("0".to_string())
                        .interact_text()
                        .unwrap()
                        .parse()
                        .unwrap();

                    circle.radius = Input::with_theme(&ColorfulTheme::default())
                        .with_prompt("Circle radius: ")
                        .default("0".to_string())
                        .interact_text()
                        .unwrap()
                        .parse()
                        .unwrap();

                    display.draw_circle(circle);
                }
                2 => {
                    let mut ellipse = Ellipse::new(Vector::new(0, 0), Vector::new(0, 0));

                    ellipse.size.x = Input::with_theme(&ColorfulTheme::default())
                        .with_prompt("Ellipse size x: ")
                        .default("0".to_string())
                        .interact_text()
                        .unwrap()
                        .parse()
                        .unwrap();

                    ellipse.size.y = Input::with_theme(&ColorfulTheme::default())
                        .with_prompt("Ellipse size y: ")
                        .default("0".to_string())
                        .interact_text()
                        .unwrap()
                        .parse()
                        .unwrap();

                    ellipse.position.x = Input::with_theme(&ColorfulTheme::default())
                        .with_prompt("Ellipse position x: ")
                        .default("0".to_string())
                        .interact_text()
                        .unwrap()
                        .parse()
                        .unwrap();

                    ellipse.position.y = Input::with_theme(&ColorfulTheme::default())
                        .with_prompt("Ellipse position y: ")
                        .default("0".to_string())
                        .interact_text()
                        .unwrap()
                        .parse()
                        .unwrap();

                    ellipse.size.x /= 2;
                    ellipse.size.y /= 2;

                    display.draw_ellipse(ellipse);
                }
                _ => break,
            },
            None => println!("Failed input"),
        }

        display.print();
        println!();
    }
}
