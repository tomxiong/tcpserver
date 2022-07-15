fn main() {
    let light = TrafficLight::Red;
    println!("Red light time is {}", light.time());
    let light = TrafficLight::Yellow;
    println!("Yellow light time is {}", light.time());
    let light = TrafficLight::Green;
    println!("Green light time is {}", light.time());
}

// The code for the first question
enum TrafficLight {
    Red,
    Green,
    Yellow,
}

trait Light {
    fn time(&self) -> u8;
}

impl Light for TrafficLight {
    fn time(&self) -> u8 {
        match &self {
            Self::Red => 30,
            Self::Yellow => 5,
            Self::Green => 45,
        }
    }
}