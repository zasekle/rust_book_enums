//Want to include
// _ => ()
// Control Flow with if let

//structs and tuples
enum Shape {
    RECTANGLE { width: i32, height: i32 },
    CIRCLE(f32),
    TRIANGLE { base: i32, height: i32 },
}

//enum impl
impl Shape {

    //match statement
    fn area(&self) -> i32 {
        match self {
            Shape::RECTANGLE { width, height } => {
                width * height
            }
            Shape::CIRCLE(radius) => {
                (std::f32::consts::PI * radius * radius) as i32
            }
            Shape::TRIANGLE { base, height } => {
                base / 2 * height
            }
        }
    }

    //Option
    fn number_sides(&self) -> Option<i32> {
        match self {
            Shape::RECTANGLE { width: _, height: _ } => {
                Some(4)
            }
            Shape::CIRCLE(_) => {
                None
            }
            Shape::TRIANGLE { base: _, height: _ } => {
                Some(3)
            }
        }
    }
}

//Wildcard
fn print_diameter(shape: &Shape) {
    match shape {
        Shape::CIRCLE(radius) => {
            println!("diameter {}", radius * 2.0);
        }
        _ => ()
    }
}

//if let
fn print_base(shape: &Shape) {
    if let Shape::TRIANGLE { base, height: _ } = shape {
        println!("base {}", base);
    };
}

fn main() {
    let triangle = Shape::TRIANGLE { base: 8, height: 12 };
    let rectangle = Shape::RECTANGLE { width: 5, height: 10 };
    let circle = Shape::CIRCLE(5.0);

    println!("triangle area {} rectangle area {} circle area {}", triangle.area(), rectangle.area(), circle.area());

    println!("triangle num sides {} rectangle num sides {} circle num sides {}", triangle.number_sides().unwrap_or(-1), rectangle.number_sides().unwrap_or(-1), circle.number_sides().unwrap_or(-1));

    print_diameter(&triangle);
    print_diameter(&rectangle);
    print_diameter(&circle);

    print_base(&triangle);
    print_base(&rectangle);
    print_base(&circle);
}
