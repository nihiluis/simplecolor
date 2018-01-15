extern crate simplecolor;

use simplecolor::{Color, Colorize};

fn main() {
    println!(
        "{} I Hope You Are Good",
        "Hello World".color(Color::Yellow)
    );
}