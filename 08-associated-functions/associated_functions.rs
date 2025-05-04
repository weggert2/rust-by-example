#![allow(unused_variables)]
#![allow(dead_code)]

struct Point {
    x: f64,
    y: f64
}

// Implementation block, all 'Point' associated functions go here
impl Point {
    // This is an "associated function" because this function is
    // associated with Point
    //
    // Associated functions don't need to be called wit an instance
    // These functions are generally used like constructors
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    // Another associated function
    fn new(x: f64, y: f64) -> Point {
        Point {x: x, y: y}
    }
}

struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    // This is a method
    // &self is sugar for self: &Self, where Self is te type of
    // the caller object. In this case, Self = Rectangle
    fn area(&self) -> f64 {
        let Point {x: x1, y: y1} = self.p1;
        let Point {x: x2, y: y2} = self.p2;

        ((x1 - x2) * (y1-y2)).abs()
    }

    fn perimeter(&self) -> f64 {
        let Point {x: x1, y: y1} = self.p1;
        let Point {x: x2, y: y2} = self.p2;

        2.0 * ((x1-x2).abs() + (y1-y2).abs())
    }

    // mut lets you change the Self passed in
    fn translate(&mut self, x: f64, y: f64) {
        self.p1.x += x;
        self.p2.x += x;

        self.p1.y += y;
        self.p2.y += y;
    }
}
// Pair owns two heap allocated integers
struct Pair(Box<i32>, Box<i32>);

impl Pair {
    // This metod "consumes" the resources of the caller object
    fn destroy(self) {
        let Pair(first, second) = self; // destructure

        println!("Destroying Pair({}, {})", first, second);

        // 'first' and 'second' go out of scope and get freed
    }
}

fn main() {
    let rectangle = Rectangle {
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };

    println!("Rectangle perimeter: {}", rectangle.perimeter());
    println!("Rectangle area: {}", rectangle.area());

    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(1.0, 1.0),
    };

    square.translate(1.0, 2.0);

    let pair = Pair(Box::new(1), Box::new(2));

    pair.destroy();
}
