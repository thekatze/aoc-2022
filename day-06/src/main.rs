use itertools::Itertools;

const INPUT: &str = include_str!("input.txt");
const WINDOW_SIZE: usize = 14;

fn main() {
    let mut window = ['\0'; WINDOW_SIZE];
    let index = INPUT
        .chars()
        .enumerate()
        .find_map(|(index, c)| {
            window[index % WINDOW_SIZE] = c;
            if index > WINDOW_SIZE && window.iter().all_unique() {
                Some(index + 1)
            } else {
                None
            }
        })
        .expect("string should have 4 consecutive characters that are distinct");

    dbg!(index);
}
