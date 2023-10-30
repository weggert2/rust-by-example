// 1, 1, 2, 3, 5, 8, 13, 21, ...
fn fib(n: u128) -> u128 {
    if n == 0 {
        return 0;
    }

    let mut curr: u128 = 1;
    let mut prev: u128 = 0;
    for _ in 1..n {
        let tmp = curr;
        curr = curr.saturating_add(prev);
        prev = tmp;
    }

    curr
}

fn main() {
    // If the user passes a number in the command line, compute that
    // fibonacci number. Otherwise, prompt them to enter a number.
    let mut input = String::new();
    if let Some(arg) = std::env::args().nth(1) {
        input = arg;
    } else {
        println!("Enter a positive number. We'll calculate that fibonacci number.");
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read user input");
    }

    let input = input.trim();
    let input_u128: u128 = input.parse().expect("Not a positive number");

    // Get the grammatical way to say a number according to English's weird rules.
    let get_one_char_suffix = |in_str: &str| match in_str.chars().last().unwrap() {
        '1' => "st",
        '2' => "nd",
        '3' => "rd",
        _ => "th",
    };

    let suffix = if input.len() >= 2 {
        match &input[input.len() - 2..] {
            "11" => "th",
            "12" => "th",
            "13" => "th",
            _ => get_one_char_suffix(input),
        }
    } else {
        get_one_char_suffix(input)
    };

    let fibret = fib(input_u128);
    println!(
        "The {input}{suffix} fibonacci number is {}{}",
        fibret,
        if fibret == u128::MAX {
            " (saturated)"
        } else {
            ""
        }
    );
}
