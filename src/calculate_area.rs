fn main() {
    println!("The third question is use generic and trait feature to calculate area of shapes:");
    println!("--------------------------------------------------------------------");
    let circle = Circle {
        radius: 10.0,
    };
    print_area(circle); // The shape area is 314

    let triangle = Triangle {
        height: 10.0,
        base: 5.0,
    };
    print_area(triangle); //The shape area is 25

    let square = Square {
        side: 10.0,
    };
    print_area(square); // The shape area is 100
    println!("--------------------------------------------------------------------");
}

// The code for third question
//-----------------------------------------------------
trait CalculateArea {
    fn area(&self) -> f32;
}

fn print_area<T: CalculateArea>(s: T) {
    println!("The shape area is {}", s.area())
}

struct Circle {
    radius: f32,
}

impl CalculateArea for Circle {
    fn area(&self) -> f32 {
        3.14 * self.radius * self.radius
    }
}

struct Triangle {
    height: f32,
    base: f32,
}

impl CalculateArea for Triangle {
    fn area(&self) -> f32 {
        (self.base * self.height) / 2.0
    }
}

struct Square {
    side: f32,
}

impl CalculateArea for Square {
    fn area(&self) -> f32 {
        self.side * self.side
    }
}
//-----------------------------------------------------