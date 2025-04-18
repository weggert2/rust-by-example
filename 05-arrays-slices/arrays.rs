fn analyze_slice(slice: &[i32]) {
    println!("first element: {}", slice[0]);
    println!("len: {}", slice.len());
}

fn main() {
    let xs: [i32; 5] = [1,2,3,4,5];
    analyze_slice(&xs);
    analyze_slice(&xs[1..3]);

    for i in 0..xs.len() + 1 {
        match xs.get(i) {
            Some(x) => println!("{}: {}", i, x),
            None => println!("Index {} is out of bounds", i),
        }
    }
}
