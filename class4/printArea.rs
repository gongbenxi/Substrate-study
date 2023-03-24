use std::fmt::Display;
// 定义一个 Trait 来约束实现该 Trait 的类型必须实现计算面积的方法
trait Area {
    fn calc_area(&self) -> f64;
}

// 定义一个圆形类型
struct Circle {
    radius: f64,
}

impl Area for Circle {
    fn calc_area(&self) -> f64 {
        std::f64::consts::PI * self.radius.powf(2.0)
    }
}

// 定义一个三角形类型
struct Triangle {
    base: f64,
    height: f64,
}

impl Area for Triangle {
    fn calc_area(&self) -> f64 {
        0.5 * self.base * self.height
    }
}

// 定义一个正方形类型
struct Square {
    side: f64,
}

impl Area for Square {
    fn calc_area(&self) -> f64 {
        self.side.powf(2.0)
    }
}

// 定义一个泛型函数，用于接收不同类型的图形作为参数，并计算它们的面积
fn print_area<T: Area + Display>(shape: T) {
    let area = shape.calc_area();
    println!("The area of the {} is {}", shape, area);
}

fn main() {
    let circle = Circle { radius: 2.0 };
    let triangle = Triangle { base: 3.0, height: 4.0 };
    let square = Square { side: 5.0 };
    
    print_area(circle);
    print_area(triangle);
    print_area(square);
}
