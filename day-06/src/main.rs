use itertools::Itertools;

const INPUT: &str = include_str!("input.txt");

fn main() {
    let index = INPUT
        .chars()
        .tuple_windows()
        .enumerate()
        .find_map(|(index, (a, b, c, d))| {
            if a == b || a == c || a == d || b == c || b == d || c == d {
                None
            } else {
                Some(index + 4)
            }
        }).expect("string should have 4 consecutive characters that are distinct");

    dbg!(index);
}
