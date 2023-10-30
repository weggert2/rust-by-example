fn times_two(x: i32) -> i32 {
    x * 2
}

fn f(x: i32) -> i32 {
    x + 1
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

    // Another way:
    let w = if y < 20 { times_two(y) } else { y };

    println!("{z}, {w}");

    println!(
        "{}",
        f({
            let y = 1;
            y + 1
        })
    );
}
