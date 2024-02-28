use std::io;

pub fn main() {
    let mut n = String::new();
    io::stdin().read_line(&mut n).unwrap();
    let n = n.trim().parse::<u64>().unwrap();
    let mut ls1 = Vec::new();
    let mut sum1 = 0;
    let mut ls2 = Vec::new();
    let mut sum2 = 0;
    for i in (1..=n).rev() {
        if sum1 > sum2 {
            ls2.push(i);
            sum2 += i;
        } else {
            ls1.push(i);
            sum1 += i;
        }
    }
    if sum1 == sum2 {
        println!("YES");
        println!("{}", ls1.len());
        println!(
            "{}",
            ls1
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        );
        println!("{}", ls2.len());
        println!(
            "{}",
            ls2
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        );
    } else {
        println!("NO");
    }
}
