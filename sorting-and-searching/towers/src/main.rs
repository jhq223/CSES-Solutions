use std::io;

fn main() {
    io::stdin().read_line(&mut String::new()).unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let nums = input
        .trim()
        .split_whitespace()
        .map(|n| n.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    println!("{}", towers(&nums));
}

fn towers(nums: &[u64]) -> u64 {
    let mut towers = Vec::new();
    for i in 0..nums.len() {
        let mut lo = 0;
        let mut hi = towers.len();
        while lo < hi {
            let mid = (lo + hi) / 2;
            if nums[i] >= towers[mid] {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }
        // If there aren't any suitable towers, append new tower to end of
        // array
        if lo == towers.len() {
            towers.push(nums[i]);
        } else {
            // If there exists a satisfying tower, add the cube to that tower
            // and update the top element of the tower
            towers[lo] = nums[i];
        }
    }
    towers.len() as u64
}

mod tests {
    use super::*;
    #[test]
    fn test_1() {
        assert_eq!(towers(&[3, 8, 2, 1, 5]), 2)
    }

    #[test]
    fn test_2() {
        assert_eq!(towers(&[1, 1, 1]), 3)
    }
}
