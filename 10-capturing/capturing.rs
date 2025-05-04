fn main() {
    use std::mem;
    let color = String::from("green");

    // A closure to print 'color', which immediately borrows('&') 'color'
    // and stores the borrow and closure in the 'print' variable.
    // It will remain borrowed until 'print' is used the last time
    //
    // 'println!' only requires arguments by immutable reference so
    // it doesn't impose anything more restrictive

    let print = || println!("color: {}", color);

    // Call the closure using the borrow
    print();

    // 'color' can be borrowed immutably again, because the
    // closure only holds an immutable reference
    let _reborrow = &color;
    print();

    // A move or rborrow is allowed after the final use of 'print'
    let _color_moved = color;

    let mut count = 0;
    // A closure to increment count could either take '&mut count' or
    // 'count', but '&mut count' is less restrictive so it takes that.
    // Thus, calling the closure mutates 'count', which requires a 'mut'
    let mut inc = || {
        count += 1;
        println!("count: {}", count);
    };

    // Call the closure using a mutable borrow
    inc();

    // The closure still mutably borrows count because it
    // is called later. An attempt to reborrow it will lead
    // to an error.
    // let _reborrow = &count;
    inc();

    // The closure no longer needs to borrow '&mut count'. Therefore,
    // it is possible to reborrow without an error.
    let _count_reborrowed = &mut count;

    // A non-copy type
    let movable = Box::new(3);

    // mem::drop requires T so tis must take by value.
    // A copy type would copy into the closure leaving the original untouched.
    // A non-copy must move so movable immediately moves into the closure
    let consume = || {
        println!("movagle: {:?}", movable);
        mem::drop(movable);
    };

    // 'consume' consumes the variable so this can only be called once
    consume();

    // Using move before vertical pipes forces closure to take ownersip
    // of the captured variables

    // Vec has non-copy semantics
    let haystack = vec![1,2,3];
    let contains = move |needle| haystack.contains(needle);

    println!("{}", contains(&1));
    println!("{}", contains(&2));
    println!("{}", contains(&3));
}
