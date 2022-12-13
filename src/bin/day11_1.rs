use lazy_static::lazy_static;
use regex::Regex;

use std::{
    cell::Cell,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let monkeys = parse_monkeys();

    for _ in 0..20 {
        for monkey in &monkeys {
            let items = monkey.items.take();
            monkey
                .visited_items
                .set(monkey.visited_items.get() + items.len());
            for item in items {
                let new_level = monkey.operation.apply(item) / 3;
                let target = monkey.test.test(new_level);

                let mut target_items = monkeys[target].items.take();
                target_items.push(new_level);
                monkeys[target].items.set(target_items);
            }
        }
    }

    let mut visited_items = monkeys
        .iter()
        .map(|m| m.visited_items.get())
        .collect::<Vec<_>>();

    visited_items.sort();

    let monkey_business = visited_items.iter().rev().take(2).product::<usize>();

    println!("monkey_business: {monkey_business}"); // 90882
}

struct Monkey {
    items: Cell<Vec<usize>>,
    visited_items: Cell<usize>,
    operation: Operation,
    test: Test,
}

#[derive(Default)]
struct MonkeyBuilder {
    items: Vec<usize>,
    operation: Option<Operation>,
    test: Option<TestBuilder>,
}

struct TestBuilder {
    division: Option<usize>,
    target_if_true: Option<usize>,
    target_if_false: Option<usize>,
}

impl MonkeyBuilder {
    fn build(self) -> Monkey {
        if let MonkeyBuilder {
            items,
            operation: Some(operation),
            test:
                Some(TestBuilder {
                    division: Some(division),
                    target_if_true: Some(target_if_true),
                    target_if_false: Some(target_if_false),
                }),
        } = self
        {
            Monkey {
                items: Cell::new(items),
                visited_items: Cell::default(),
                operation,
                test: Test {
                    division,
                    target_if_true,
                    target_if_false,
                },
            }
        } else {
            panic!("Incomplete monkey !")
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Operation {
    Mul(usize),
    Add(usize),
    Square,
}

impl Operation {
    fn apply(self, value: usize) -> usize {
        match self {
            Self::Mul(v) => value * v,
            Self::Add(v) => value + v,
            Self::Square => value * value,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Test {
    division: usize,
    target_if_true: usize,
    target_if_false: usize,
}

impl Test {
    fn test(self, value: usize) -> usize {
        if value % self.division == 0 {
            self.target_if_true
        } else {
            self.target_if_false
        }
    }
}

fn parse_monkeys() -> Vec<Monkey> {
    let f = File::open("inputs/day11").unwrap();
    let reader = BufReader::new(f);

    reader
        .lines()
        .map(|l| l.unwrap())
        .map(|l| Token::parse(&l))
        .chain(vec![Token::EndMonkey])
        .scan(MonkeyBuilder::default(), |current_monkey, token| {
            match token {
                Token::EndMonkey => {
                    let monkey = std::mem::take(current_monkey);
                    return Some(Some(monkey.build()));
                }
                Token::StartMonkey(_) => {}
                Token::Items(items) => current_monkey.items = items,
                Token::Operation(operation) => current_monkey.operation = Some(operation),
                Token::StartTest(division) => {
                    current_monkey.test = Some(TestBuilder {
                        division: Some(division),
                        target_if_true: None,
                        target_if_false: None,
                    })
                }
                Token::TestOk(target) => {
                    current_monkey.test.as_mut().unwrap().target_if_true = Some(target)
                }
                Token::TestNok(target) => {
                    current_monkey.test.as_mut().unwrap().target_if_false = Some(target)
                }
            }

            Some(None)
        })
        .flatten()
        .collect::<Vec<_>>()
}

#[derive(Debug)]
enum Token {
    StartMonkey(usize),
    Items(Vec<usize>),
    Operation(Operation),
    StartTest(usize),
    TestOk(usize),
    TestNok(usize),
    EndMonkey,
}

impl Token {
    fn parse(line: &str) -> Self {
        lazy_static! {
            static ref START_MONKEY: Regex = Regex::new(r"Monkey (\d+):").unwrap();
            static ref ITEMS: Regex = Regex::new(r"  Starting items: ((\d+)(,\s)?)+").unwrap();
            static ref INDICES: Regex = Regex::new(r"(\d+)").unwrap();
            static ref OPERATION: Regex =
                Regex::new(r"  Operation: new = old ([*|+]) (.+)").unwrap();
            static ref START_TEST: Regex = Regex::new(r"  Test: divisible by (\d+)").unwrap();
            static ref TEST_OK: Regex = Regex::new(r"    If true: throw to monkey (\d+)").unwrap();
            static ref TEST_NOK: Regex =
                Regex::new(r"    If false: throw to monkey (\d+)").unwrap();
        }

        if let Some(captures) = START_MONKEY.captures(line) {
            let index = captures.get(1).unwrap().as_str().parse::<usize>().unwrap();
            Self::StartMonkey(index)
        } else if ITEMS.is_match(line) {
            let items = INDICES
                .captures_iter(line)
                .map(|c| c.get(0).unwrap().as_str().parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            Self::Items(items)
        } else if let Some(captures) = OPERATION.captures(line) {
            let operator = captures.get(1).unwrap().as_str();
            let value = captures.get(2).unwrap().as_str();

            match operator {
                "*" => {
                    if let Ok(value) = value.parse::<usize>() {
                        Self::Operation(Operation::Mul(value))
                    } else {
                        Self::Operation(Operation::Square)
                    }
                }
                "+" => Self::Operation(Operation::Add(value.parse::<usize>().unwrap())),
                _ => panic!(),
            }
        } else if let Some(captures) = START_TEST.captures(line) {
            let value = captures.get(1).unwrap().as_str().parse::<usize>().unwrap();
            Self::StartTest(value)
        } else if let Some(captures) = TEST_OK.captures(line) {
            let value = captures.get(1).unwrap().as_str().parse::<usize>().unwrap();
            Self::TestOk(value)
        } else if let Some(captures) = TEST_NOK.captures(line) {
            let value = captures.get(1).unwrap().as_str().parse::<usize>().unwrap();
            Self::TestNok(value)
        } else if line.is_empty() {
            Self::EndMonkey
        } else {
            panic!()
        }
    }
}
