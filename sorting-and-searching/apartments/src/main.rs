use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    io::stdin().read_line(&mut input3).expect("Failed to read input");
    let k = input1
        .trim()
        .split_whitespace()
        .map(|n| n.parse::<i32>())
        .nth(2)
        .unwrap()
        .unwrap();
    let ls1 = input2
        .trim()
        .split_whitespace()
        .map(|n| n.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    let ls2 = input3
        .trim()
        .split_whitespace()
        .map(|n| n.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    println!("{}", apartments(ls1, ls2, k));
}

fn apartments(mut applicants: Vec<i32>, mut apartments: Vec<i32>, k: i32) -> u32 {
    applicants.sort();
    apartments.sort();
    let mut res = 0_u32;
    let mut l1 = 0;
    let mut l2 = 0;
    while l1 < applicants.len() && l2 < apartments.len() {
        if apartments[l2] >= applicants[l1] - k && apartments[l2] <= applicants[l1] + k {
            res += 1;
            l1 += 1;
            l2 += 1;
        } else {
            if apartments[l2] < applicants[l1] - k {
                l2 += 1;
            } else if apartments[l2] > applicants[l1] + k {
                l1 += 1;
            } else {
                l1 += 1;
                l2 += 1;
            }
        }
    }
    res
}
