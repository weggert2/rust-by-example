fn reverse(pair: (i32, bool)) -> (bool, i32) {
    (pair.1, pair.0)
}

fn main() {
    let reversed = reverse((4, true));
    println!("{}, {}", reversed.0, reversed.1);
}
