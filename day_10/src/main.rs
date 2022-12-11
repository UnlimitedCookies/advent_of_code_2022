#![feature(iter_collect_into)]

fn main() {
    let input = std::fs::read_to_string("input").unwrap();

    // first challenge
    let mut cycle_count: i32 = 1; // because we count during the 20th, 60th, 100th.. cycle
    let mut register_x: i32 = 1; // predefined initial value
    let signal_strength_sum: i32 = input.lines().filter_map(|line| {
        match &line[0..4] {
            "addx" => { cycle_count += 2; register_x += line[5..].parse::<i32>().unwrap(); },
            "noop" => cycle_count += 1,
            _ => (),
        }
        match cycle_count {
            cycle if (cycle - 19) % 40 == 0 => Some(register_x * (cycle + 1)),
            cycle if (cycle - 20) % 40 == 0 && line != "noop" => Some(register_x * cycle),
            _ => None,
        }
    }).sum();
    println!("Signal strength sum: {signal_strength_sum}");

    // second challenge
    let mut cycle_count: usize = 1; // because we count during the 20th, 60th, 100th.. cycle
    let mut register_x: i32 = 1; // predefined initial value
    let mut clock_x_map: Vec<_> = vec![(1, 1)]; // starting position for x is 1
    input.lines().map(|line| {
        match &line[0..4] {
            "addx" => { cycle_count += 2; register_x += line[5..].parse::<i32>().unwrap(); },
            "noop" => cycle_count += 1,
            _ => (),
        }
        (cycle_count, register_x)
    }).collect_into(&mut clock_x_map);
    
    let mut pixmap = [[' '; 40]; 6];
    pixmap.iter_mut().flat_map(|row| row.iter_mut()).enumerate().for_each(|(i, char)| {
        let (_, sprite_pos) = clock_x_map.iter().rfind(|e| e.0 == i + 1 || e.0 == i).unwrap();
        let adjusted_sprite_pos = sprite_pos + ((i / 40) * 40) as i32;
        *char = match i as i32 {
            i if i >= adjusted_sprite_pos -1 && i<= adjusted_sprite_pos + 1 => '#',
            _ => '.',
        }
    });

    let mut bitmap_string = String::new();
    pixmap.iter().for_each(|row| {
        row.iter().collect_into(&mut bitmap_string);
        bitmap_string.push('\n');
    });
    println!("CRT output:\n{bitmap_string}");
}
