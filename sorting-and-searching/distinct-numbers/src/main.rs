use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut String::new()).expect("Failed to read input");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let mut res = 1;
    let mut numbers: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|n| n.parse::<i32>().unwrap())
        .collect();
    numbers.sort();

    for w in numbers.windows(2) {
        if w[1] != w[0] {
            res += 1;
        }
    }

    println!("{}", res);
}
