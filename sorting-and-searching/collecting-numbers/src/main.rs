use std::io;

fn main() {
    let mut input = String::new();
    let mut n = String::new();
    io::stdin().read_line(&mut n).unwrap();
    io::stdin().read_line(&mut input).unwrap();
    let n = n.trim().parse::<u64>().unwrap();
    let nums = input
        .trim()
        .split_whitespace()
        .map(|n| n.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();
    println!("{}", collecting_numbers(&nums, n));
}

fn collecting_numbers(nums: &[u64], n: u64) -> u64 {
    let mut flags = vec![0_usize; (n+1)as usize];
    for (i, v) in nums.iter().enumerate() {
        flags[*v as usize] = i;
    }
    let mut res = 0;
    let mut index = 0_usize;
    while (index as u64) < n {
        index += 1;
        res += 1;
        while index + 1 < flags.len() && flags[index] < flags[index + 1] {
            index += 1;
        }
    }
    res
}
