use std::io;

pub fn main() {
    let mut n = String::new();
    io::stdin().read_line(&mut n).unwrap();
    let n = n.trim().parse::<u32>().expect("parse error: n");
    for _ in 0..n {
        let mut num = String::new();
        io::stdin().read_line(&mut num).unwrap();
        let num = num.trim().parse::<u64>().expect("parse error: num");
        println!("{}", digit_queries(num));
    }
}

fn digit_queries(p: u64) -> u32 {
    let mut len: u64 = 1;
    let mut p = p;
    loop {
        let g = (10_u64).pow((len - 1) as u32) * 9 * len;
        if p > g {
            p -= g;
            len += 1;
        } else {
            break;
        }
    }
    let mut index = p % len;
    let mut num = (10_u64).pow((len - 1) as u32) + p / len;
    if index == 0 {
        num -= 1;
        index = len - 1;
    } else {
        index -= 1;
    }
    let num = num.to_string().chars().into_iter().collect::<Vec<char>>();
    let res = num
        .get(index as usize)
        .unwrap()
        .to_digit(10)
        .unwrap();
    res
}
