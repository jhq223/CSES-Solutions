use std::io;

pub fn main() {
    let mut n = String::new();
    io::stdin().read_line(&mut n).unwrap();
    let _ = n.trim().parse::<u32>();
    let mut apples = String::new();
    io::stdin().read_line(&mut apples).unwrap();
    let apples = apples
        .split_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();
    println!("{}", apple_division(&apples));
}

fn apple_division(apples: &[u64]) -> u64 {
    let sum = apples.iter().sum::<u64>();
    let mut ans = u64::MAX;
    let lens = apples.len();
    for i in 0..1 << lens {
        let mut temp = Vec::new();
        for j in 0..lens {
            if (i & (1 << j)) != 0 {
                temp.push(apples[j]);
            }
        }
        let t_sum = temp.iter().sum::<u64>();
        ans = ans.min((sum - t_sum).abs_diff(t_sum));
    }
    ans
}
