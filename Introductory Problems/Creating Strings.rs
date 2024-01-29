use std::{ collections::HashSet, io };

pub fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let choice = input.trim().chars().collect::<Vec<char>>();
    let mut res = create_string(&choice);
    res.sort();
    println!("{}", res.len());
    for i in res {
        let s = i.iter().collect::<String>();
        println!("{}", s);
    }
}

fn backtrack(
    mut state: Vec<char>,
    choices: &[char],
    selected: &mut Vec<bool>,
    result: &mut Vec<Vec<char>>
) {
    if state.len() == choices.len() {
        result.push(state);
        return;
    }
    let mut duplicated = HashSet::<char>::new();
    for i in 0..choices.len() {
        let choice = &choices[i];
        if !selected[i] && !duplicated.contains(choice) {
            duplicated.insert(*choice);
            selected[i] = true;
            state.push(*choice);
            backtrack(state.clone(), choices, selected, result);
            selected[i] = false;
            state.pop();
        }
    }
}

fn create_string(choices: &[char]) -> Vec<Vec<char>> {
    let mut result = Vec::new();
    backtrack(Vec::new(), choices, &mut vec![false; choices.len()], &mut result);
    result
}
