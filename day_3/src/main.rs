fn main() {
    let input = std::fs::read_to_string("input").unwrap();

    // first challenge
    let total_priorities: u32 = input.lines().map(|line| {
        let line_split = line.split_at(line.len() / 2);
        let double_char = line_split.0.chars().find(|char|
            line_split.1.contains(*char)).unwrap();
        if (double_char as u8) < 91 {
            double_char as u32 - 38
        } else {
            double_char as u32 - 96
        }
    }).sum();
    println!("Total priorities: {total_priorities}");

    // second challenge
    let lines: Vec<_> = input.lines().collect();
    let total_priorities: u32 = lines.chunks(3).map(|line| {
        let badge_char = line[0].chars().find(|char|
            line[1].contains(*char) && line[2].contains(*char)).unwrap();
        if (badge_char as u8) < 91 {
            badge_char as u32 - 38
        } else {
            badge_char as u32 - 96
        }
    }).sum();
    println!("Total Elf group badge priorities: {total_priorities}");
}
