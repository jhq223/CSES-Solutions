use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    io::stdin().read_line(&mut input3).expect("Failed to read input");
    let ls1 = to_vec::<u32>(&input2);
    let ls2 = to_vec::<u32>(&input3);
    let res = concert_tickets(ls1, ls2);
    for i in res {
        match i {
            Some(n) => println!("{}", n),
            None => println!("-1"),
        }
    }
}

fn concert_tickets(mut ls1: Vec<u32>, ls2: Vec<u32>) -> Vec<Option<u32>> {
    ls1.sort();
    // ls2.sort();

    let mut res = vec![];
    let mut map = std::collections::BTreeMap::new();
    for i in ls1 {
        map.entry(i).or_insert(1);
    }
    for i in ls2 {
        match map.upper_bound(std::ops::Bound::Included(&i)).key() {
            Some(&n) => {
                res.push(Some(n));
                map.entry(n).and_modify(|v| {
                    *v -= 1;
                });
                if map[&n] == 0 {
                    map.remove(&n);
                }
            }
            None => {
                res.push(None);
            }
        }
    }
    res
}

fn to_vec<T: std::str::FromStr>(s: &str) -> Vec<T>
    where <T as std::str::FromStr>::Err: std::fmt::Debug
{
    s.trim()
        .to_string()
        .split_whitespace()
        .map(|n| n.parse::<T>().unwrap())
        .collect()
}
