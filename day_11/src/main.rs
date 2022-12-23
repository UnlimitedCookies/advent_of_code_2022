use std::collections::VecDeque;

#[derive(Debug, Clone)]
struct Monkey {
    items: VecDeque<u64>, // could also be a Vec, but I like to do it in order with pop_front()
    inspection_count: u64,
    operation: (Operator, Option<u64>),
    test: MonkeyTest,
}

#[derive(Debug, Clone)]
struct MonkeyTest {
    divisible_by: u64,
    if_true: usize,
    if_false: usize,
}

#[derive(Debug, Clone)]
enum Operator {
    Add,
    Multiply,
}

impl Monkey {
    fn apply_operation(&self, item: u64) -> u64 {
        self.operation.0.apply(item, self.operation.1.unwrap_or(item))
    }
}

impl Operator {
    fn apply(&self, lhs: u64, rhs: u64) -> u64 {
        match self {
            Self::Add => lhs + rhs,
            Self::Multiply => lhs * rhs,
        }
    }
}

impl std::str::FromStr for Operator {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Self::Add),
            "*" => Ok(Self::Multiply),
            _ => Err("Invalid Operator character found.".into()),
        }
    }
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let mut monkeys: Vec<Monkey> = input.split("\n\n").map(|monkey| {
        let monkey_builder: Vec<_> = monkey.lines().collect();
        Monkey {
            items: monkey_builder[1][18..].split_terminator(", ").map(|item| item.parse::<_>().unwrap()).collect(),
            inspection_count: 0,
            operation: (monkey_builder[2][23..24].parse::<Operator>().unwrap(), monkey_builder[2][25..].parse().ok()),
            test: MonkeyTest { divisible_by: monkey_builder[3][21..].parse().unwrap(), if_true: monkey_builder[4][29..].parse().unwrap(), if_false: monkey_builder[5][30..].parse().unwrap() },
        }
    }).collect();
    let mut monkeys2 = monkeys.clone();

    // first challenge
    (0..20).for_each(|_| {
        (0..monkeys.len()).for_each(|i| {
            while let Some(mut item) = monkeys[i].items.pop_front() {
                monkeys[i].inspection_count += 1;
                item = monkeys[i].apply_operation(item) / 3;
                let destination_monkey = if item % monkeys[i].test.divisible_by == 0 {
                     monkeys[i].test.if_true
                } else {
                    monkeys[i].test.if_false
                };
                monkeys[destination_monkey].items.push_back(item);
            }
        });
    });

    println!("20 rounds with division by 3 - level of monkey business: {}", determine_monkey_business(monkeys));

    // second challenge
    let common_denominator = monkeys2.iter().fold(1, |acc, monkey| acc * monkey.test.divisible_by);
    (0..10000).for_each(|_| {
        (0..monkeys2.len()).for_each(|i| {
            while let Some(mut item) = monkeys2[i].items.pop_front() {
                monkeys2[i].inspection_count += 1;
                item = monkeys2[i].apply_operation(item);
                item %= common_denominator; // to keep the numbers low
                let destination_monkey = if item % monkeys2[i].test.divisible_by == 0 {
                     monkeys2[i].test.if_true
                } else {
                    monkeys2[i].test.if_false
                };
                monkeys2[destination_monkey].items.push_back(item);
            }
        });
    });


    println!("10000 rounds without division by 3 - level of monkey business: {}", determine_monkey_business(monkeys2));
    
    fn determine_monkey_business(monkeys: Vec<Monkey>) -> u64 {
        let max_inspection_count = monkeys.iter().map(|monkey| monkey.inspection_count).max().unwrap();
        let second_to_largest_count = monkeys.iter().filter_map(|monkey| {
            if monkey.inspection_count < max_inspection_count { Some(monkey.inspection_count) } else { None }
        }).max().unwrap();
        max_inspection_count * second_to_largest_count
    }
}
