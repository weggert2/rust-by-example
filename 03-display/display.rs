use std::fmt;

struct MyStruct(i32);
struct Point2D {
    x: f64,
    y: f64,
}

struct MyList(Vec<i32>);

impl fmt::Display for MyStruct {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}, {}", self.x, self.y)
    }
}

impl fmt::Display for MyList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vec = &self.0;
        write!(f, "[")?;

        for (count, v) in vec.iter().enumerate() {
            if count != 0 {
               write!(f, ", ")?;
            }
            write!(f, "{}", v)?;
        }

        write!(f, "]")
    }
}

fn main() {
    let my_struct: MyStruct = MyStruct(42);
    println!("{}", my_struct);

    let point: Point2D = Point2D {
        x: 3.0,
        y: 4.0,
    };

    println!("{}", point);

let my_list: MyList = MyList(vec![1,2,3]);
    println!("{}", my_list);
}
