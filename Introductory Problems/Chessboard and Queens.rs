fn main() {
    let res = eight_queens(8);
    println!("{}", res);
}

fn backtrack(
    state: &mut Vec<Vec<String>>,
    row: usize,
    n: usize,
    cols: &mut [bool],
    diags1: &mut [bool],
    diags2: &mut [bool],
    res: &mut u32
) {
    if row == n {
        *res += 1;
        return;
    }

    for col in 0..n {
        let diag1 = row + col;
        let diag2 = row + n - col - 1;
        if !cols[col] && !diags1[diag1] && !diags2[diag2] && state[row][col] != "*" {
            state[row][col] = "Q".to_string();
            cols[col] = true;
            diags1[diag1] = true;
            diags2[diag2] = true;
            backtrack(state, row + 1, n, cols, diags1, diags2, res);
            state[row][col] = ".".to_string();
            cols[col] = false;
            diags1[diag1] = false;
            diags2[diag2] = false;
        }
    }
}

fn eight_queens(n: usize) -> u32 {
    let mut chessboard = Vec::with_capacity(8);
    for _ in 0..8 {
        let mut temp_input = String::new();
        std::io::stdin().read_line(&mut temp_input).unwrap();
        chessboard.push(
            temp_input
                .trim()
                .chars()
                .map(|c| c.to_string())
                .collect::<Vec<String>>()
        );
    }

    let mut cols = vec![false; n];
    let mut diags1 = vec![false; 2 * n - 1];
    let mut diags2 = vec![false; 2 * n - 1];

    let mut res = 0;

    backtrack(&mut chessboard, 0, n, &mut cols, &mut diags1, &mut diags2, &mut res);
    return res;
}
