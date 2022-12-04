fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    
    // first challenge
    let fully_contains_count: u32 = input.lines().map(|line|{
        let elf_pair = line.split_once(",").unwrap();
        let elf1_string = elf_pair.0.split_once("-").unwrap();
        let elf2_string = elf_pair.1.split_once("-").unwrap();
        let elf1_range: (u32, u32) = (elf1_string.0.parse().unwrap(), elf1_string.1.parse().unwrap());
        let elf2_range: (u32, u32) = (elf2_string.0.parse().unwrap(), elf2_string.1.parse().unwrap());

        if (elf1_range.0 <= elf2_range.0 && elf1_range.1 >= elf2_range.1) || (elf1_range.0 >= elf2_range.0 && elf1_range.1 <= elf2_range.1) {
            1
        } else {
            0
        }
    }).sum();
    println!("Total amount of contained pairs: {fully_contains_count}");

    // second challenge
    let total_overlap_count: u32 = input.lines().map(|line|{
        let elf_pair = line.split_once(",").unwrap();
        let elf1_string = elf_pair.0.split_once("-").unwrap();
        let elf2_string = elf_pair.1.split_once("-").unwrap();
        let elf1_range: (u32, u32) = (elf1_string.0.parse().unwrap(), elf1_string.1.parse().unwrap());
        let elf2_range: (u32, u32) = (elf2_string.0.parse().unwrap(), elf2_string.1.parse().unwrap());

        if elf1_range.0 <= elf2_range.1 && elf1_range.1 >= elf2_range.0 {
            1
        } else {
            0
        }
    }).sum();
    println!("Total amount of overlapping pairs: {total_overlap_count}");
}
