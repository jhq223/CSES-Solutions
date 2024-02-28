use std::io;

pub fn main() {
    let mut n = String::new();
    io::stdin().read_line(&mut n).unwrap();
    let n = n.trim().parse::<u64>().unwrap();
    for _ in 0..n {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        let line = line
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();
        if coin_piles(line[0], line[1]) {
            println!("YES");
        } else {
            println!("NO");
        }
    }
}

fn coin_piles(a: u64, b: u64) -> bool {
    (a + b) % 3 == 0 && a.min(b) * 2 >= a.max(b)
}

mod tests {
    #[test]
    fn coin_piles_test() {
        assert_eq!(super::coin_piles(1, 2), true);
        assert_eq!(super::coin_piles(3, 3), true);
    }
}
