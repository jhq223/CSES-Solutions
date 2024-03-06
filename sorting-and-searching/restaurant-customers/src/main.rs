use std::io;

fn main() {
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("input error");
    let n: u32 = n.trim().parse().expect("parse error");
    let mut map = std::collections::BTreeMap::new();
    for _ in 0..n {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("input error");
        let input = input
            .trim()
            .split_whitespace()
            .map(|n| n.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();
        map.insert(input[0], 1);
        map.insert(input[1], -1);
    }
    let mut res = 0;
    let mut sum = 0;
    for (_, v) in map {
        sum += v;
        res = res.max(sum);
    }

    println!("{}", res);
}
