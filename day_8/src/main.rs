fn main() {
    let input = std::fs::read_to_string("input").unwrap();

    // parse the input
    let tree_grid: Vec<Vec<_>> = input.lines().map(|line| {
        line.chars().map(|char| char.to_digit(10).unwrap()).collect()
    }).collect();

    // first challenge
    let visible_tree_count: u32 = (0..tree_grid[0].len()).map(|x| {
        let column_vec: Vec<_> = (0..tree_grid.len()).map(|i| tree_grid[i][x]).collect();
        (0..tree_grid.len()).map(|y|{
            let tree = tree_grid[y][x];
            let row_visible = tree_grid[y].iter().take(x).all(|e| e < &tree) || tree_grid[y].iter().skip(x + 1).all(|e| e < &tree);
            let column_visible = column_vec.iter().take(y).all(|e| e < &tree) || column_vec.iter().skip(y + 1).all(|e| e < &tree);
            if row_visible || column_visible { 1 } else { 0 }
        }).sum::<u32>()
    }).sum();

    println!("Amount of visible trees: {visible_tree_count}");

    // second challenge
    let highest_scenic_score = (0..tree_grid[0].len()).map(|x| {
        let column_vec: Vec<_> = (0..tree_grid.len()).map(|i| tree_grid[i][x]).collect();
        (0..tree_grid.len()).map(|y|{
            let tree = tree_grid[y][x];
            let mut exit_filter = false;
            let left = tree_grid[y].iter().take(x).rev().filter(|e| { if exit_filter { return false; } if !(e < &&tree) { exit_filter = true; } true }).count();
            exit_filter = false;
            let right = tree_grid[y].iter().skip(x + 1).filter(|e| { if exit_filter { return false; } if !(e < &&tree) { exit_filter = true; } true }).count();
            exit_filter = false;
            let up = column_vec.iter().take(y).rev().filter(|e| { if exit_filter { return false; } if !(e < &&tree) { exit_filter = true; } true }).count();
            exit_filter = false;
            let down = column_vec.iter().skip(y + 1).filter(|e| { if exit_filter { return false; } if !(e < &&tree) { exit_filter = true; } true }).count();
            left * right * up * down
        }).max().unwrap()
    }).max().unwrap();
    println!("Highest scenic score possible: {highest_scenic_score}");
}
