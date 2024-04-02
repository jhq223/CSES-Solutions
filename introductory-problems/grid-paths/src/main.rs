use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let in_paths = input.trim().chars().collect::<Vec<char>>();

    let mut vis = vec![vec![false; 9]; 9];
    for i in 0..=8 {
        vis[0][i] = true;
        vis[i][8] = true;
        vis[8][i] = true;
        vis[i][0] = true;
    }

    let mut res: u32 = 0;
    dfs(1, 1, 0, &in_paths, &mut vis, &mut res);
    println!("{}", res);
}

fn dfs(
    r: usize,
    c: usize,
    s: usize,
    in_paths: &Vec<char>,
    vis: &mut Vec<Vec<bool>>,
    res: &mut u32
) {
    let dx: [i32; 4] = [0, 1, 0, -1];
    let dy: [i32; 4] = [1, 0, -1, 0];

    if vis[r][c] {
        return;
    }

    // 遇到墙必定会分两部分
    if !vis[r][c + 1] && !vis[r][c - 1] && vis[r + 1][c] && vis[r - 1][c] {
        return;
    }
    if !vis[r + 1][c] && !vis[r - 1][c] && vis[r][c + 1] && vis[r][c - 1] {
        return;
    }

    // 被包围，必定会损失其一
    if !vis[r + 1][c + 1] && vis[r + 1][c] && vis[r][c + 1] {
        return;
    }
    if !vis[r + 1][c - 1] && vis[r + 1][c] && vis[r][c - 1] {
        return;
    }
    if !vis[r - 1][c - 1] && vis[r - 1][c] && vis[r][c - 1] {
        return;
    }
    if !vis[r - 1][c + 1] && vis[r - 1][c] && vis[r][c + 1] {
        return;
    }

    if r == 7 && c == 1 {
        if s == 48 {
            *res += 1;
        }
        return;
    }

    if s == 48 {
        return;
    }

    vis[r][c] = true;

    match in_paths[s] {
        'L' => dfs(r, c - 1, s + 1, in_paths, vis, res),
        'R' => dfs(r, c + 1, s + 1, in_paths, vis, res),
        'U' => dfs(r - 1, c, s + 1, in_paths, vis, res),
        'D' => dfs(r + 1, c, s + 1, in_paths, vis, res),
        _ => {
            for i in 0..4 {
                dfs(
                    ((r as i32) + dx[i]) as usize,
                    ((c as i32) + dy[i]) as usize,
                    s + 1,
                    in_paths,
                    vis,
                    res
                );
            }
        }
    }

    vis[r][c] = false;
}
