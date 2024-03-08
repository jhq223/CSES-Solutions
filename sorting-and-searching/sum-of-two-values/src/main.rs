use std::collections::HashMap;
use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();
    io::stdin().read_line(&mut input1).unwrap();
    io::stdin().read_line(&mut input2).unwrap();
    let l1 = input1
        .trim()
        .split_whitespace()
        .map(|n| n.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    let l2 = input2
        .trim()
        .split_whitespace()
        .map(|n| n.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    match search(&l2, l1[1]) {
        Some(res) => println!("{} {}", res[0], res[1]),
        None => println!("IMPOSSIBLE"),
    }
}

fn search(nums: &[u32], target: u32) -> Option<[usize; 2]> {
    let mut map = HashMap::new();
    for (i, num) in nums.iter().enumerate() {
        if map.contains_key(num) && target == 2 * num {
            return Some([map[num], i + 1]);
        }
        map.insert(*num, i + 1);
    }

    for num in nums {
        if map.contains_key(&(target - num)) && target != 2 * num {
            return Some([map[num], map[&(target - num)]]);
        }
    }
    None
}
