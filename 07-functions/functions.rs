fn main() {
    fizzbuzz_to(100);
}

// Functions that "don't" return a value actually
// return the unit type '()'
fn fizzbuzz(n: u32) -> () {
    if is_divisible_by(n, 15) {
        println!("fizzbuzz");
    } else if is_divisible_by(n, 3) {
        println!("fizz");
    } else if is_divisible_by(n, 5) {
        println!("buzz");
    } else {
        println!("{}", n);
    }
}

// When a function returns '()', you can exclude it
// from the signature
fn fizzbuzz_to(n: u32) {
    for n in 1..=n {
        fizzbuzz(n); // uses the local n
    }
}

fn is_divisible_by(lhs: u32, rhs: u32) -> bool {
    if rhs == 0 {
        return false;
    }

    let divisible: bool = lhs % rhs == 0;
    divisible  // return without the return keyword,
               // because rust wants to make sense.
}
