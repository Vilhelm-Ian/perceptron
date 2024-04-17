use crate::rand::gen_range;
use macroquad::prelude::*;

struct Point {
    x: f32,
    y: f32,
    color: Color,
}

impl Point {
    fn random_new<F>(guess: F) -> Self
    where
        F: Fn(f32, f32) -> Color,
    {
        let x = gen_range(0.0, 500.0);
        let y = gen_range(0.0, 500.0);
        let color = guess(x, y);
        Self { x, y, color }
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

    fn guess(&self, inputs: &Vec<f32>) -> i32 {
        let sum: f32 = inputs
            .iter()
            .enumerate()
            .map(|(index, input)| input * self.weights[index])
            .sum();
        activation_function(sum)
    }

    fn return_color(&self, x: f32, y: f32) -> Color {
        if self.guess(&vec![x, y]) == 1 {
            BLUE
        } else {
            YELLOW
        }
    }

    fn train(&mut self, inputs: &Vec<f32>, target: i32) {
        let guess = self.guess(inputs);
        let error = target - guess;
        println!("{:?} {:?}", error, self.weights);
        self.weights
            .iter_mut()
            .enumerate()
            .for_each(|(index, weight)| {
                *weight += error as f32 * inputs[index] * 0.1;
            });
    }
}

#[macroquad::main("BasicShapes")]
async fn main() {
    let points_1 = (0..100)
        .map(|_| Point::random_new(hand_written_function))
        .collect::<Vec<Point>>();
    let mut perceptron = Perceptron::new();
    println!("{:?}", perceptron.weights);
    let points_2 = (0..100)
        .map(|_| Point::random_new(|x, y| perceptron.return_color(x, y)))
        .collect::<Vec<Point>>();
    points_1.iter().for_each(|point| {
        let target = if point.x + point.y > 500.0 { -1 } else { 1 };
        perceptron.train(&vec![point.x, point.y], target);
    });
    let points_3 = (0..100)
        .map(|_| Point::random_new(|x, y| perceptron.return_color(x, y)))
        .collect::<Vec<Point>>();
    println!("{:?}", perceptron.weights);
    loop {
        clear_background(RED);
        draw_line(0.0, 500.0, 500.0, 0.0, 1.0, BLACK);
        draw_line(500.0, 0.0, 500.0, 1000.0, 1.0, BLACK);
        draw_line(0.0, 500.0, 1000.0, 500.0, 1.0, BLACK);
        points_1.iter().for_each(|point| {
            draw_rectangle(point.x, point.y, 10.0, 10.0, point.color);
        });
        points_2.iter().for_each(|point| {
            draw_rectangle(500.0 + point.x, point.y, 10.0, 10.0, point.color);
        });
        points_3.iter().for_each(|point| {
            draw_rectangle(point.x, 500.0 + point.y, 10.0, 10.0, point.color);
        });
        next_frame().await
    }
}

fn activation_function(input: f32) -> i32 {
    if input > 0.0 {
        1
    } else {
        -1
    }
}

fn hand_written_function(x: f32, y: f32) -> Color {
    if x + y > 500.0 {
        BLUE
    } else {
        YELLOW
    }
}
