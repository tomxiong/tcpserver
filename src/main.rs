
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
    // Normal case
    let ints1:Vec<u32> = vec![1112121211, 2334232346];
    println!("Total value is :{:?}", sum_u_ints(&ints1));
    // overflow case
    let ints2:Vec<u32> = vec![1112121211, 2334232346, 1112121211, 23342323];
    println!("Wrong value is :{:?}", sum_u_ints(&ints2));
    println!("--------------------------------------------------------------------");

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

// The code for the first question
enum TrafficLight {
    Red,
    Green,
    Yellow,
}

//define the trait for time of traffic light
trait Light {
    fn time(&self) -> u8;
}

// implement the trait for enum
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
    // init the total with zero first
    let mut total = 0_u32;
    // for loop all items of slice
    for u in items {
        // add each value for total
        match total.checked_add(*u) {
            //Set the new value to total if it is not overflow
            Some(v) => {
                total = v;
            }
            //if it is overflow,then print error and return None
            None => {
                println!("overflow!");
               return None
            }
        }
    }
    // return the total value if not overflow
    Some(total)
}
//-----------------------------------------------------

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
