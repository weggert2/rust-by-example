#![allow(dead_code)]

struct Person {
    name: String,
    age: u8,
}

// Apparently field-less structs are useful for generics.
struct Unit;

struct Pair(i32, i32);

struct Point {
    x: f32,
    y: f32,
}

struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

fn main() {
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };

    let point1 = Point { x: 5.2, y: 0.2 };
    let point2 = Point { x: 10.3, y: 11.3 };


}
