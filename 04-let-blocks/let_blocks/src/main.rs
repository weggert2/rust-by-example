fn times_two(x: i32) -> i32 {
    x * 2
}

fn main() {
    let y = 10;
    let z = {
        if y < 20 {
            times_two(y)
        } else {
            y
        }
    };

    println!("{z}");
}
