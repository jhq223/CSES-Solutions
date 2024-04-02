use std::io;

pub fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n = input.trim().parse::<u32>().unwrap();
    let res = gray_code(n);
    for r in res {
        println!("{}", r);
    }
}

fn gray_code(n: u32) -> Vec<String> {
    let mut res = vec![];
    for i in 0..(2_i32).pow(n) {
        let mut s = format!("{:b}", i ^ (i >> 1));
        if s.len() < (n as usize) {
            s.insert_str(0, &"0".repeat((n as usize) - s.len()));
        }
        res.push(s);
    }
    res
}
