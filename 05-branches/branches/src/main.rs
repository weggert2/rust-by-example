fn print_range<T: std::iter::Iterator>(r: T)
where
    <T as Iterator>::Item: std::fmt::Display,
    <T as Iterator>::Item: std::fmt::Debug,
    // Note the below also works, if you use {:?} instead of {i}:
    // T: Iterator,
    // T::Item: std::fmt::Debug,
{
    for i in r {
        println!("{i}");
    }
}

fn main() {
    let mut i = 0;
    let result = loop {
        println!("{i}");
        i += 1;
        if i > 10 {
            break i;
        }
        std::thread::sleep(std::time::Duration::from_millis(10));
    };

    println!("The final value is {result}");

    // Loop labels. They have a leading apostrophe for some reason.
    i = 0;
    'outer: loop {
        i += 1;
        loop {
            i *= 2;
            if i > 100 {
                break 'outer;
            }
        }
    }

    println!("The new final value is {i}");

    let x = [1, 2, 3, 4, 5];
    println!("{}", x.len());

    let r = 1..=4;
    println!("printing range");
    print_range(r);
}
