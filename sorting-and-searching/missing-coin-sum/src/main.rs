fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut String::new()).unwrap();
    std::io::stdin().read_line(&mut input).unwrap();
    let coins = input
        .trim()
        .split_whitespace()
        .map(|n| n.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();
    println!("{}", missing_coin_sum(coins));
}

fn missing_coin_sum(mut coins: Vec<u64>) -> u64 {
    coins.sort();
    let mut res = 1;
    for coin in coins {
        if coin <= res {
            res += coin;
        } else {
            break;
        }
    }
    res
}
