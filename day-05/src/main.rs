use std::{
    borrow::BorrowMut,
    fmt::{Display, Write},
};

use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    combinator::map,
    multi::{separated_list1},
    sequence::{delimited, preceded, tuple},
    Finish, IResult, Parser,
};

const INPUT: &str = include_str!("input.txt");

#[derive(Debug)]
struct Instruction {
    amount: usize,
    from: usize,
    to: usize,
}

#[derive(Debug)]
struct CrateYard(Box<[Vec<char>]>);
impl CrateYard {
    fn apply(&mut self, instruction: &Instruction) {
        let from: &mut Vec<char> = self.borrow_mut().0[instruction.from - 1].borrow_mut();

        let from_index = from.len().saturating_sub(instruction.amount);
        let removed: Vec<char> = from.drain(from_index..from.len()).collect();

        let to: &mut Vec<char> = self.borrow_mut().0[instruction.to - 1].borrow_mut();
        to.extend(removed);
    }
}

impl Display for CrateYard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (index, crate_row) in self.0.iter().enumerate() {
            f.write_char(' ')?;
            f.write_char((index + 1).to_string().chars().next().unwrap())?;
            f.write_char(' ')?;
            for crate_ in crate_row {
                f.write_char(' ')?;
                f.write_char(*crate_)?;
            }
            f.write_char('\n')?;
        }

        for (_, crate_row) in self.0.iter().enumerate() {
            f.write_char(*crate_row.last().unwrap())?;
        }

        Ok(())
    }
}

fn main() {
    let (mut crate_yard, instructions) = parse_input(INPUT).expect("input to be correct");

    for instruction in instructions.iter() {
        crate_yard.apply(instruction);
    }

    println!("{}", crate_yard);
}

// ALL THE PARSING LOGIC

fn transpose_yard<T>(v: Vec<Vec<Option<T>>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .rev()
                .filter_map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

fn parse_input(input: &str) -> Result<(CrateYard, Box<[Instruction]>), ()> {
    let (crate_yard, instructions) = input.split_once("\n\n").ok_or(())?;

    let (_input, crate_yard) = parse_crate_yard(crate_yard).expect("valid crate yard");

    let crate_yard = CrateYard(transpose_yard(crate_yard).into_boxed_slice());

    let (_, instructions) = parse_instructions(instructions)
        .finish()
        .expect("valid instructions");

    Ok((crate_yard, instructions.into_boxed_slice()))
}

fn parse_crate_yard(input: &str) -> IResult<&str, Vec<Vec<Option<char>>>> {
    separated_list1(tag("\n"), parse_crate_row)(input)
}

fn parse_crate_row(input: &str) -> IResult<&str, Vec<Option<char>>> {
    separated_list1(tag(" "), parse_crate_or_none)(input)
}

fn parse_crate_or_none(input: &str) -> IResult<&str, Option<char>> {
    alt((map(parse_crate, Some), map(parse_none, |_| None)))(input)
}

fn parse_none(input: &str) -> IResult<&str, &str> {
    take(3_u32)(input)
}

fn parse_crate(input: &str) -> IResult<&str, char> {
    map(
        delimited(tag("["), nom::character::complete::alpha1, tag("]")),
        |character: &str| character.chars().next().expect("crate to contain char"),
    )(input)
}

fn parse_instructions(instructions: &str) -> IResult<&str, Vec<Instruction>> {
    separated_list1(tag("\n"), parse_instruction)(instructions)
}

fn parse_instruction(instruction: &str) -> IResult<&str, Instruction> {
    map(
        tuple((
            preceded(tag("move "), nom::character::complete::u32),
            preceded(tag(" from "), nom::character::complete::u32),
            preceded(tag(" to "), nom::character::complete::u32),
        )),
        |(amount, from, to)| Instruction {
            amount: amount as usize,
            from: from as usize,
            to: to as usize,
        },
    )(instruction)
}
