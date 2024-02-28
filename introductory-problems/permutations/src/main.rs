use std::io;

pub fn main() {
    let mut n = String::new();
    io::stdin().read_line(&mut n).unwrap();
    let n = n.trim().parse::<u32>().unwrap();
    let nums = (1..=n).collect::<Vec<u32>>();
    let nums_1 = nums
        .iter()
        .filter(|&&x| x % 2 != 0)
        .collect::<Vec<&u32>>();
    let mut nums_2 = nums
        .iter()
        .filter(|&&x| x % 2 == 0)
        .collect::<Vec<&u32>>();
    nums_2.extend(nums_1);
    let no_solution = nums_2.windows(2).any(|x| x[0].abs_diff(*x[1]) == 1);
    if no_solution {
        println!("NO SOLUTION");
    } else {
        println!(
            "{}",
            nums_2
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        );
    }
}
