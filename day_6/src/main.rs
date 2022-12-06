fn main() {
    let input = std::fs::read_to_string("input").unwrap();

    // first challenge 
    println!("First packet marker detected after processing {} charactes.", get_after_marker_index(&input, 4));

    // second challenge
    println!("First message marker detected after processing {} charactes.", get_after_marker_index(&input, 14));

    fn get_after_marker_index(input: &str, marker_size: usize) -> usize {
        input.as_bytes().windows(marker_size).enumerate().find(|(i, chars)|{
            !(1..chars.len()).any(|i| {
                chars[i..].contains(&chars[i - 1])
            })
        }).unwrap().0 + marker_size
    }
}
