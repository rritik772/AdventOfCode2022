#![allow(non_snake_case)]

pub fn run(lines: Vec<Vec<char>>) -> u32 {
    return second(lines);
}

#[derive(PartialEq, PartialOrd, Clone)]
enum Move {
    Rock = 1,
    Paper = 2,
    Scissor = 3
}

enum Round {
    Lose = 0,
    Draw = 3,
    Win = 6
}

impl Move {
    fn convert(m: &char) -> Self {
        match m {
            'X' | 'A' => Move::Rock,
            'Y' | 'B' => Move::Paper,
            _ => Move::Scissor,
        }
    }

    fn i_have_to(m: &char) -> Round {
        match m {
            'X' => Round::Lose,
            'Y' => Round::Draw,
            _ => Round::Win
        }
    }

    fn win_map(o: Move, m: Move) -> Round {
        return if o == Move::Rock && m == Move::Paper { Round::Win } 
        else if o == Move::Rock && m == Move::Scissor { Round::Lose }
        else if o == Move::Paper && m == Move::Scissor { Round::Win }
        else if o == Move::Paper && m == Move::Rock { Round::Lose }
        else if o == Move::Scissor && m == Move::Rock { Round::Win }
        else if o == Move::Scissor && m == Move::Paper { Round::Lose }
        else {Round::Draw}
    }

    fn to_win(o: Self) -> Self {
        match o {
            Move::Rock => Move::Paper,
            Move::Paper => Move::Scissor,
            Move::Scissor => Move::Rock
        }
    }

    fn to_lose(o: Self) -> Self {
        match o {
            Move::Rock => Move::Scissor,
            Move::Paper => Move::Rock,
            Move::Scissor => Move::Paper
        }
    }
}

fn first(lines: Vec<Vec<char>>) -> u32 {
    let mut total = 0;

    for round in lines {
        let other = Move::convert(round.get(0).unwrap());
        let me = Move::convert(round.get(1).unwrap());

        let status = Move::win_map(other, me.to_owned());

        total += status as u32 + me as u32;
    }

    return total;
}

fn second(lines: Vec<Vec<char>>) -> u32 {
    let mut total = 0;
    
    for round in lines {
        let other = Move::convert(round.get(0).unwrap());
        let i_have_to = Move::i_have_to(round.get(1).unwrap());


        let status = match i_have_to {
            Round::Win => Move::to_win(other.to_owned()),
            Round::Lose => Move::to_lose(other.to_owned()),
            Round::Draw => other.to_owned()
        };

        let points = Move::win_map(other, status.to_owned());

        total += status as u32 + points as u32;
    }

    return total;
}
