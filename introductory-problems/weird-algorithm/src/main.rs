pub fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let n: u64 = input.trim().parse().unwrap();
    let mut res = collatz(n).unwrap();
    res.push(n);
    res.reverse();
    println!(
        "{}",
        res
            .iter()
            .map(|i| i.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
}

pub fn collatz(n: u64) -> Option<Vec<u64>> {
    match n {
        0 => None,
        1 => Some(Vec::new()),
        n if n % 2 == 0 =>
            collatz(n / 2).map(|mut res: Vec<u64>| {
                res.push(n / 2);
                res
            }),
        n =>
            collatz(n.checked_mul(3)?.checked_add(1)?).map(|mut res: Vec<u64>| {
                res.push(3 * n + 1);
                res
            }),
    }
}
