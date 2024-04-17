use crate::rand::gen_range;
use macroquad::prelude::*;

struct Point {
    x: f32,
    y: f32,
}

impl Point {
    fn random_new() -> Self {
        Self {
            x: gen_range(0.0, 500.0),
            y: gen_range(0.0, 500.0),
        }
    }
}

struct Perceptron {
    weights: Vec<f32>,
}

impl Perceptron {
    fn new() -> Self {
        Self {
            weights: (0..2).map(|_| gen_range(-1.0, 1.0)).collect(),
        }
    }

    fn guess(&self, inputs: Vec<f32>) -> i32 {
        let sum: f32 = inputs
            .iter()
            .enumerate()
            .map(|(index, input)| input * self.weights[index])
            .sum();
        if sum > 0.0 {
            1
        } else {
            -1
        }
    }
}

#[macroquad::main("BasicShapes")]
async fn main() {
    let points = (0..100)
        .map(|_| Point::random_new())
        .collect::<Vec<Point>>();
    loop {
        clear_background(RED);
        draw_line(0.0, 0.0, 500.0, 500.0, 1.0, BLACK);
        points.iter().for_each(|point| {
            let color = if point.x > point.y { BLUE } else { YELLOW };
            draw_rectangle(point.x, point.y, 10.0, 10.0, color);
        });
        next_frame().await
    }
}
