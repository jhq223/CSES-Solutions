use std::io;

pub fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n = input.trim().parse::<u32>().unwrap();
    gray_code(n);
}

fn gray_code(n: u32) {
    let print_num = |n: &Vec<u32>| {
        println!(
            "{}",
            n
                .iter()
                .map(|x| x.to_string())
                .collect::<String>()
        );
    };

    let change_num = |i: u32| {
        if i == 0 {
            return 1;
        } else {
            return 0;
        }
    };
    let mut num = std::iter
        ::repeat(0)
        .take(n as usize)
        .collect::<Vec<u32>>();
    print_num(&num);
    for _ in 0..(2_i32).pow(n - 1) - 1 {
        if let Some(last) = num.get_mut((n - 1) as usize) {
            *last = change_num(*last);
        }
        print_num(&num);
        for i in (1..num.len()).rev() {
            if num[i] == 1 {
                num[i - 1] = change_num(num[i - 1]);
                break;
            }
        }
        print_num(&num);
    }
    if let Some(last) = num.get_mut((n - 1) as usize) {
        *last = change_num(*last);
    }
    print_num(&num);
}
