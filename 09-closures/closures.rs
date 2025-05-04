fn main() {
    let outer_var = 42;

    // A regular cuntion can' trefer to variables in the enclosing environemnt
    // fn function(i: i32) -> i32 { i + outer_var }

    // Closures are anonymous. Here we are binding them to references.
    // Annotation is ientical to function annotation, but is optional
    // as are the {} wrapping the body. These nameless functions are
    // assigned to appropriately named variables.

    let closure_annotated = |i: i32| -> i32 { i + outer_var };
    let closure_inferred  = |i    |          i + outer_var;

    // Call the closures
    println!("closure annotated: {}", closure_annotated(1));
    println!("closure inferred:  {}", closure_inferred(1));

    let one = || 1;
    println!("closure returning 1: {}", one());
}
