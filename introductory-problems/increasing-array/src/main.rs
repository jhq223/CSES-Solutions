use std::io;

pub fn main() {
    let mut n = String::new();
    let mut input = String::new();
    io::stdin().read_line(&mut n).unwrap();
    io::stdin().read_line(&mut input).unwrap();
    let _ = n.trim().parse::<u32>().unwrap();
    let mut nums = input
        .trim()
        .split_whitespace()
        .map(|i| i.parse::<u128>().unwrap())
        .collect::<Vec<u128>>();

    let mut result = 0;
    for i in 0..nums.len() - 1 {
        if nums[i] > nums[i + 1] {
            result += nums[i] - nums[i + 1];
            nums[i + 1] = nums[i];
        }
    }
    println!("{}", result);
}
