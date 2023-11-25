const INPUT: &str = include_str!("input.txt");

struct Range<T> {
    from: T,
    to: T,
}

impl TryFrom<&str> for Range<u32> {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (from, to) = value
            .split_once('-')
            .ok_or("range does not have correct delimiter")?;

        Ok(Self {
            from: from.parse().map_err(|_| "from is not a number")?,
            to: to.parse().map_err(|_| "to is not a number")?,
        })
    }
}

impl<T> Range<T>
where
    T: PartialOrd<T>,
{
    fn contains(&self, value: &T) -> bool {
        self.from <= *value && *value <= self.to
    }

    fn partially_overlaps(&self, other: &Range<T>) -> bool {
        self.contains(&other.from)
            || self.contains(&other.to)
            || other.contains(&self.from)
            || other.contains(&self.to)
    }
}

fn main() {
    let partial_overlaps = INPUT
        .lines()
        .filter_map(|range_definitions| {
            let (left, right) = range_definitions
                .split_once(',')
                .expect("comma separated ranges");

            let (left, right) = (
                Range::<u32>::try_from(left).expect("left input to be correct"),
                Range::<u32>::try_from(right).expect("right input to be correct"),
            );

            if left.partially_overlaps(&right) {
                Some(())
            } else {
                None
            }
        })
        .count();

    dbg!(partial_overlaps);
}
