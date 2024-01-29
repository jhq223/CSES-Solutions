use std::io;

pub fn main() {
    let mut n = String::new();
    io::stdin().read_line(&mut n).unwrap();
    let n = n.trim().parse::<u32>().unwrap();
    for _ in 0..n {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        let line = line
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<u128>().unwrap())
            .collect::<Vec<u128>>();
        println!("{}", get_num(line[1] - 1, line[0] - 1));
    }
}

fn get_num(x: u128, y: u128) -> u128 {
    let max = x.max(y);
    let num = (max + 1).pow(2) - max;
    let flag = max % 2 == 0;
    match (x.cmp(&y), flag) {
        (std::cmp::Ordering::Equal, _) => num,
        (std::cmp::Ordering::Less, true) => num - (y - x),
        (std::cmp::Ordering::Less, false) => num + (y - x),
        (std::cmp::Ordering::Greater, true) => num + (x - y),
        (std::cmp::Ordering::Greater, false) => num - (x - y),
    }
}

mod tests {
    #[test]
    fn test_get_num() {
        assert_eq!(super::get_num(0, 0), 1);
        assert_eq!(super::get_num(2, 1), 8);
        assert_eq!(super::get_num(1, 2), 6);
    }
}
