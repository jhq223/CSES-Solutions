use std::{ collections::HashMap, io };

pub fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim().chars().collect::<Vec<char>>();
    let mut map: HashMap<char, u32> = HashMap::new();
    for i in input {
        *map.entry(i).or_default() += 1;
    }

    let is_solution =
        map
            .values()
            .filter(|&&v| v % 2 != 0)
            .count() <= 1;
    if is_solution {
        let mut result = String::new();
        let mut repeat = String::new();
        for (c, v) in map {
            if v % 2 != 0 {
                repeat.push(c);
                result.insert_str(
                    result.len() / 2,
                    std::iter
                        ::repeat(c)
                        .take((v - 1) as usize)
                        .collect::<String>()
                        .as_str()
                );
            } else {
                result.insert_str(
                    result.len() / 2,
                    std::iter
                        ::repeat(c)
                        .take(v as usize)
                        .collect::<String>()
                        .as_str()
                );
            }
        }
        result.insert_str(result.len() / 2, &repeat);
        println!("{}", result);
    } else {
        println!("NO SOLUTION")
    }
}
