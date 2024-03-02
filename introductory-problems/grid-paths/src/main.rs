use std::io;

// !!!!!! TLE not pass
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

    // 边界检查
    if r == 0 || r == 8 || c == 0 || c == 8 {
        return;
    }

    // 访问过
    if vis[r][c] {
        return;
    }

    // 到达终点
    if s == 48 {
        if r == 7 && c == 1 {
            *res += 1;
        }
        return;
    }

    // 必定失败的情况
    if
        (!vis[r][c + 1] && !vis[r][c - 1] && vis[r + 1][c] && vis[r - 1][c]) ||
        (!vis[r + 1][c] && !vis[r - 1][c] && vis[r][c + 1] && vis[r][c - 1])
    {
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
