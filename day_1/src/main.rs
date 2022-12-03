fn main() {
    let input = std::fs::read_to_string("input").unwrap();

    // first challenge
    let mut calories_total_counts: Vec<u32> = input.split("\n\n").map(|multi_lines| 
        multi_lines.lines().map(|y| y.parse::<u32>().unwrap()).sum()).collect();
    let maximum_count = calories_total_counts.iter().max().unwrap();
    println!("Maximum calorie count: {maximum_count}");

    // second challenge
    calories_total_counts.sort_unstable();
    let top_3_count: _ = calories_total_counts.iter().rev().take(3).sum::<u32>();
    println!("Top 3 calorie count: {top_3_count}");
}
