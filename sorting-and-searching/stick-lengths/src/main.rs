use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut String::new()).unwrap();
    io::stdin().read_line(&mut input).unwrap();
    let nums = input
        .trim()
        .split_whitespace()
        .map(|n| n.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    println!("{}", stick_length(nums));
}

fn stick_length(mut sticks: Vec<i64>) -> i64 {
    sticks.sort();
    if sticks.len() == 1 {
        return 0;
    }
    let mid1 = sticks[sticks.len() / 2];
    let mid2 = sticks[sticks.len() / 2 - 1];
    let res1 = sticks.iter().fold(0, |acc, &stick| acc + (stick - mid1).abs());
    let res2 = sticks.iter().fold(0, |acc, &stick| acc + (stick - mid2).abs());
    res1.min(res2)
}
