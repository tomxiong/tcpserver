fn main() {
    println!("The third question is use generic and trait feature to calculate area of shapes:");
    println!("--------------------------------------------------------------------");
    // init circle object with value
    let circle = Circle {
        radius: 10.0,
    };
    // call function to print the area
    print_area(circle); // The shape area is 314

    // init triangle object with value
    let triangle = Triangle {
        height: 10.0,
        base: 5.0
    };
    // call function to print the area
    print_area(triangle); //The shape area is 25

    // init square object with value
    let square = Square {
        side: 10.0,
    };
    // call function to print the area
    print_area(square); // The shape area is 100
    println!("--------------------------------------------------------------------");
}

// The code for third question
//-----------------------------------------------------
// define the trait for calculate area
trait CalculateArea {
    fn area(&self) -> f32;
}

// Call area method for generic object to print the area of the shape object
fn print_area<T:CalculateArea>(s:T) {
    println!("The shape area is {}", s.area())
}

// circle object struct
struct Circle {
    radius: f32,
}

// implement the calculate area trait for circle
impl CalculateArea for Circle {
    fn area(&self) -> f32 {
        3.14*self.radius*self.radius
    }
}

// triangle object struct
struct Triangle {
    height: f32,
    base: f32,
}
// implement the calculate area trait for triangle
impl CalculateArea for Triangle {
    fn area(&self) -> f32 {
        (self.base*self.height)/2.0
    }
}

// square object struct
struct Square {
    side: f32,
}
// implement the calculate area trait for Square
impl CalculateArea for Square {
    fn area(&self) -> f32 {
        self.side*self.side
    }
}
//-----------------------------------------------------