const INPUT: &str = include_str!("input.txt");

fn main() {
    let mut sorted = INPUT
        .split("\n\n")
        .map(|block| {
            block
                .lines()
                .filter_map(|num| num.parse::<u32>().ok())
                .sum::<u32>()
        })
        .collect::<Vec<u32>>();

    sorted.sort_by(|a, b| b.cmp(a));

    let result: u32 = sorted.iter().take(3).sum();
    dbg!(result);
}
