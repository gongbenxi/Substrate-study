// 定义一个枚举表示交通信号灯的三种状态
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

// 定义一个 trait 表示可以返回时间的对象
trait Timed {
    fn time(&self) -> u8;
}

// 为 TrafficLight 实现 Timed trait
impl Timed for TrafficLight {
    fn time(&self) -> u8 {
        match *self {
            TrafficLight::Red => 30,
            TrafficLight::Yellow => 5,
            TrafficLight::Green => 45,
        }
    }
}

fn main() {
    let red = TrafficLight::Red;
    let yellow = TrafficLight::Yellow;
    let green = TrafficLight::Green;
    
    println!("Red light lasts for {} seconds", red.time());
    println!("Yellow light lasts for {} seconds", yellow.time());
    println!("Green light lasts for {} seconds", green.time());
}
