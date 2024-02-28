use std::io;

pub fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let nums = input.trim().chars().collect::<Vec<char>>();
    let mut max_len = 0;

    let mut low = 0;
    let mut high = 0;

    while low <= high && high < nums.len() {
        if nums[low] == nums[high] {
            high += 1;
        } else {
            max_len = max_len.max(high - low);
            low = high;
        }
    }
    max_len = max_len.max(high - low);
    println!("{}", max_len);
}
