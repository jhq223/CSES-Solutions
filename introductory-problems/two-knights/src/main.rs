use std::io;

pub fn main() {
    let mut n = String::new();
    io::stdin().read_line(&mut n).unwrap();
    let n = n.trim().parse::<u32>().unwrap();
    println!("{}", 0);
    for i in 2..n + 1 {
        let i = i as u128;
        let num: u128 = (i.pow(2) * (i.pow(2) - 1)) / 2 - 4 * (i - 1) * (i - 2);
        println!("{}", num);
    }
}
