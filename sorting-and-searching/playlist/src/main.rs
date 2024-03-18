use std::collections::HashSet;
use std::io;

fn main() {
    let mut t = String::new();
    io::stdin().read_line(&mut t).unwrap();
    let _t: u32 = t.trim().parse().unwrap();
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    let nums = s
        .split_whitespace()
        .map(|n| n.parse::<u32>().unwrap())
        .collect::<Vec<_>>();
    println!("{}", paly_list(&nums));
}

fn paly_list(nums: &[u32]) -> u32 {
    let mut low = 0;
    let mut high = 0;
    let mut set = HashSet::new();
    let mut max = 0;
    while high < nums.len() {
        while high < nums.len() && !set.contains(&nums[high]) {
            set.insert(nums[high]);
            high += 1;
        }
        max = max.max(high - low);
        // println!("{:?}", set);
        while high < nums.len() && nums[low] != nums[high] {
            set.remove(&nums[low]);
            low += 1;
        }
        // println!("{} {}", low, high);
        low += 1;
        high += 1;
        //set.remove(&nums[low]);
    }
    max as u32
}
