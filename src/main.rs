
fn main()  {
    println!("The first question is the time of the different color traffic light:");
    println!("--------------------------------------------------------------------");
    let light = TrafficLight::Red;
    println!("Red light time is {}", light.time());
    let light = TrafficLight::Yellow;
    println!("Yellow light time is {}", light.time());
    let light = TrafficLight::Green;
    println!("Green light time is {}", light.time());
    println!("--------------------------------------------------------------------");

    println!("The second question is summing of vec u32 values:");
    println!("--------------------------------------------------------------------");
    let ints:Vec<u32> = vec![1112121211, 2334232346, 1112121211, 23342323];
    let total_ints = sum_u_ints(&ints);
    println!("Total value is :{}", total_ints.unwrap());
    println!("--------------------------------------------------------------------");

    println!("The third question is use generic and trait feature to calculate area of shapes:");
    println!("--------------------------------------------------------------------");
    let circle = Circle {
        radius: 10.0,
    };
    print_area(circle); // The shape area is 314

    let triangle = Triangle {
        height: 10.0,
        base: 5.0
    };
    print_area(triangle); //The shape area is 25

    let square = Square {
      side: 10.0,
    };
    print_area(square); // The shape area is 100
    println!("--------------------------------------------------------------------");
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

// The code for the second question
//-----------------------------------------------------
fn sum_u_ints(items: &[u32]) -> Option<u32> {
    let mut total = 0_u32;
    for u in items {
        match total.checked_add(*u) {
            Some(v) => {
                total = v;
            }
            None => {
                println!("overflow!");
                ()
            }
        }
    }
    Some(total)
}
//-----------------------------------------------------

// The code for third question
//-----------------------------------------------------
trait CalculateArea {
    fn area(&self) -> f32;
}

fn print_area<T:CalculateArea>(s:T) {
    println!("The shape area is {}", s.area())
}

struct Circle {
    radius: f32,
}

impl CalculateArea for Circle {
    fn area(&self) -> f32 {
        3.14*self.radius*self.radius
    }
}

struct Triangle {
    height: f32,
    base: f32,
}

impl CalculateArea for Triangle {
    fn area(&self) -> f32 {
        (self.base*self.height)/2.0
    }
}

struct Square {
    side: f32,
}

impl CalculateArea for Square {
    fn area(&self) -> f32 {
        self.side*self.side
    }
}
//-----------------------------------------------------
