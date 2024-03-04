use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();

    io::stdin().read_line(&mut input1).expect("Failed to read input");
    io::stdin().read_line(&mut input2).expect("Failed to read input");

    let nx = input1
        .trim()
        .split_whitespace()
        .map(|n| n.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    let mut ps = input2
        .trim()
        .split_whitespace()
        .map(|n| n.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    ps.sort();

    println!("{}", ferris_wheel(nx[1], ps));
}

fn ferris_wheel(x: u32, ps: Vec<u32>) -> u32 {
    let mut res = 0;
    let mut l = 0;
    let mut r = ps.len() - 1;
    while l <= r {
        if ps[l] + ps[r] <= x {
            l += 1;
            r -= 1;
        } else {
            if r == 0 {
                return res + 1;
            }
            r -= 1;
        }
        res += 1;
    }
    res
}
