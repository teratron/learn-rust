use std::f32::consts::PI;

trait Shape {
    // У любой формы можно посчитать площадь.
    fn area(&self) -> f32;
}

trait HasAngles: Shape {
    // У любой фигуры с углами можно посчитать количество углов.
    fn angles_count(&self) -> i32;
}

struct Rectangle {
    x: f32,
    y: f32,
}

// Прямоугольник является формой.
impl Shape for Rectangle {
    fn area(&self) -> f32 {
        self.x * self.y
    }
}

// Прямоугольник является фигурой с углами.
impl HasAngles for Rectangle {
    fn angles_count(&self) -> i32 {
        4
    }
}

struct Circle {
    r: f32,
}

// Круг является формой
impl Shape for Circle {
    fn area(&self) -> f32 {
        self.r.powi(2) * PI
    }
}

fn main() {
    println!("Hello, world!");
}
