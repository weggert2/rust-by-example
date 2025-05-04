// F must implement Fn for a closure which takes no inputs
// and returns noting - exactly what is required for print
fn apply<F>(f: F)
    where F: Fn()
{
    f();
}

fn main()
{
    let x = 7;
    let print = || println!("{}", x);
    apply(print);

    // Define a closure satisfying the Fn bound
    let closure = || println!("I am a closure");

    call_me(closure);
    call_me(function);
}

// define a function which takes a generic F argument
// bounded by Fn and calls it
fn call_me<F: Fn()>(f: F) {
    f();
}

// Define a wrapper function satisfying the Fn bound
fn function() {
    println!("I am a function");
}

