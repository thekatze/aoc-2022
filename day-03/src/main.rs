use itertools::Itertools;

const INPUT: &str = include_str!("input.txt");

trait PriorityScore {
    fn priority_score(&self) -> u32;
}

impl PriorityScore for char {
    fn priority_score(&self) -> u32 {
        match self {
            'a'..='z' => *self as u32 - ('a' as u32 - 1),
            'A'..='Z' => *self as u32 - ('A' as u32 - 1) + 26,
            _ => unreachable!("Invalid item in rucksack"),
        }
    }
}

const CHUNK_SIZE: usize = 3;
fn main() {
    let priorities = INPUT
        .lines()
        .chunks(CHUNK_SIZE)
        .into_iter()
        .map(|group| {
            group
                // map every line to a bit mask of items in the rucksack
                .map(|rucksack| {
                    rucksack
                        .chars()
                        .fold(0 as u64, |acc, item| acc | 1 << item.priority_score())
                })
                // and the bit mask to find the item thats in every rucksack
                .fold(u64::MAX, |acc, current| acc & current)
                // check which item it was
                .trailing_zeros()
        })
        .sum::<u32>();
    dbg!(priorities);
}
