fn main() {
    let input = std::fs::read_to_string("input").unwrap();

    // first challenge
    let total_points: u32 = input.lines().map(|line| {
        let (opponent_move, my_move) = line.split_once(" ").unwrap();
        let translated_char = match opponent_move {
            "A" => "X",
            "B" => "Y",
            "C" => "Z",
            _ => "",
        };
        let win_points = match (opponent_move, my_move) {
            _ if (translated_char == my_move) => 3,
            ("A", "Y") => 6,
            ("B", "Z") => 6,
            ("C", "X") => 6,
            _ => 0,
        };
        let bonus_points = match my_move {
            "X" => 1,
            "Y" => 2,
            "Z" => 3,
            _ => 0,
        };
        win_points + bonus_points
    }).sum();
    println!("predefined move - total points: {total_points}");

    // second challenge
    let total_points: u32 = input.lines().map(|line| {
        let (opponent, win_condition) = line.split_once(" ").unwrap();
        let win_points = match win_condition {
            "X" => 0,
            "Y" => 3,
            "Z" => 6,
            _ => 0,
        };
        let bonus_points = match opponent {
            "A" => match win_condition {
                    "X" => 3, // Scissors
                    "Y" => 1, // Rock
                    "Z" => 2, // Paper
                    _ => 0,
            },
            "B" => match win_condition {
                "X" => 1, // Rock
                "Y" => 2, // Paper
                "Z" => 3, // Scissors
                _ => 0,
            },
            "C" => match win_condition {
                "X" => 2, // Paper
                "Y" => 3, // Scissors
                "Z" => 1, // Rock
                _ => 0,
            },
            _ => 0,
        };
        win_points + bonus_points
    }).sum();
    println!("predefined win condition - total points: {total_points}");
}
