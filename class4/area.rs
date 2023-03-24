use std::fmt::Display;

// 定义一个可以计算面积的trait
trait Area {
    fn area(&self) -> f64;
}

// 定义一个可以打印面积的函数，使用泛型和泛型约束
fn print_area<T: Area + Display>(shape: &T) {
    println!("The area of the {} is {}", shape, shape.area());
}

// 定义一个圆形结构体，实现面积计算方法
struct Circle {
    radius: f64,
}

impl Area for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

impl Display for Circle {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "circle with radius {}", self.radius)
    }
}

// 定义一个三角形结构体，实现面积计算方法
struct Triangle {
    base: f64,
    height: f64,
}

impl Area for Triangle {
    fn area(&self) -> f64 {
        0.5 * self.base * self.height
    }
}

impl Display for Triangle {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "triangle with base {} and height {}", self.base, self.height)
    }
}

// 定义一个正方形结构体，实现面积计算方法
struct Square {
    side: f64,
}

impl Area for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

impl Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "square with side length {}", self.side)
    }
}

fn main() {
    let circle = Circle { radius: 2.0 };
    let triangle = Triangle { base: 3.0, height: 4.0 };
    let square = Square { side: 5.0 };

    print_area(&circle);
    print_area(&triangle);
    print_area(&square);
}
