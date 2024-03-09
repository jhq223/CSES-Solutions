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
    println!("{}", max_subarray_sum(&nums));
}

fn max_subarray_sum(nums: &[i64]) -> i64 {
    let mut dp = vec![0; nums.len()];
    let mut res = nums[0];
    dp[0] = nums[0];
    for i in 1..nums.len() {
        dp[i] = nums[i].max(nums[i] + dp[i - 1]);
        res = res.max(dp[i]);
    }
    res
}
