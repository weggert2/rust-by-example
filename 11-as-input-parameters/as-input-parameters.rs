// A function which takes a closure as an argument and calls it.
// <F> denotes that F is a generic type parameter
fn apply<F>(f: F)
    // The closure takes no input and returns nothing.
    // FnOnce is a trait. The version of the call operator that
    // takes a by-value receiver
    where F: FnOnce()
{
    // Fn or FnMut (also traits) wouldn't work here.
    f();
}

// A function which takes a closure and returns an i32
fn apply_to_3<F>(f: F) ->i32
    where F: Fn(i32) -> i32
{
    f(3)
}

fn main() {
    use std::mem;

    let greeting = "hello";

    // A non-copy type
    // to-owned creates owned data from a borrowed one
    let mut farewell = "goodbye".to_owned();

    // Capture 2 variables: 'greeting' by reference and
    // 'farewell' by value
    let diary = || {
        // 'greeting' is by reference: requires 'Fn'
        println!("I said {}.", greeting);

        // Mutation forces 'farewell' to be captured
        // by mutable reference. Now requires 'FnMut'
        farewell.push_str("!!!");
        println!("Then I screamed {}.", farewell);

        // Manually calling drop forces 'farewell' to
        // be captured by value. Now requires 'FnOnce'
        mem::drop(farewell);
    };

    // Cal the function which applies the closure
    apply(diary);

    // 'double' satisfies apply_to_3's trait bound
    let double = |x| 2 * x;
    println!("3 doubled: {}", apply_to_3(double));

}
