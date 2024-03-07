use std::io;

fn main() {
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).unwrap();
    let n = input1.trim().parse::<usize>().unwrap();
    let mut ls = Vec::new();
    for _ in 0..n {
        let mut input2 = String::new();
        io::stdin().read_line(&mut input2).unwrap();
        let v: Vec<u32> = input2
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect();
        ls.push((v[0], v[1]));
    }
    ls.sort_by(|(_, a), (_, b)| a.cmp(b));

    let mut res = 0;
    let mut time = 0;
    for (s, e) in ls {
        if s >= time {
            res += 1;
            time = e;
        }
    }
    println!("{}", res);
}
