fn main() {
    let input = std::fs::read_to_string("input").unwrap();

    // first challenge 
    let (input_stack, input_moves) = input.split_once("\n\n").unwrap();
    let (input_stack, _) = input_stack.split_once("\n 1").unwrap();
    let mut stacks: Vec<Vec<_>> = vec![Vec::new(); 9];

    input_stack.lines().rev().for_each(|line| {
        line.chars().skip(1).step_by(4).enumerate().for_each(|(i, char)| {
            if char.is_ascii_alphabetic() {
                stacks[i].push(char);
            }
        });
    });
    let mut stacks2 = stacks.clone();

    input_moves.lines().for_each(|line| {
        let quantity: usize = line[5..line.find("from").unwrap() - 1].parse().unwrap();
        let from_index: usize = line[line.find("from").unwrap() + 5..line.find("to").unwrap() - 1].parse().unwrap();
        let to_index: usize = line[line.find("to").unwrap() + 3..].parse().unwrap();
        (0..quantity).into_iter().for_each(|_| {
            let element = stacks[from_index - 1].pop().unwrap();
            stacks[to_index - 1].push(element);
        });
    });


    let top_crates: String = stacks.iter().map(|stack| {
        stack.last().unwrap()
    }).collect();
    println!("Crates on top: {top_crates}");

    // second challenge
    input_moves.lines().for_each(|line| {
        let quantity: usize = line[5..line.find("from").unwrap() - 1].parse().unwrap();
        let from_index: usize = line[line.find("from").unwrap() + 5..line.find("to").unwrap() - 1].parse().unwrap();
        let to_index: usize = line[line.find("to").unwrap() + 3..].parse().unwrap();
        let element_range_index = stacks2[from_index - 1].len() - quantity;
        let mut elements: Vec<_> = stacks2[from_index - 1].drain(element_range_index..).collect();
        stacks2[to_index - 1].append(& mut elements);
    });

    let top_crates: String = stacks2.iter().map(|stack| {
        stack.last().unwrap()
    }).collect();
    println!("Crates on top: {top_crates}");
}
