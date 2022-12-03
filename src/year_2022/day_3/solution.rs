use crate::common::inputs::challenge_test_suite;

enum Rps {
    Rock,
    Paper,
    Scissors,
}

impl From<&str> for Rps {
    fn from(str: &str) -> Self {
        match str {
            "A" | "X" => Rps::Rock,
            "B" | "Y" => Rps::Paper,
            "C" | "Z" => Rps::Scissors,
            _ => panic!("unknown char \"{}\"", str),
        }
    }
}

impl From<Rps> for usize {
    fn from(rps: Rps) -> Self {
        match rps {
            Rps::Rock => 1,
            Rps::Paper => 2,
            Rps::Scissors => 3,
        }
    }
}

enum RpsWinner {
    Me,
    Opponent,
    Draw,
}

fn check_winner(me: &Rps, opponent: &Rps) -> RpsWinner {
    match (me, opponent) {
        (Rps::Rock, Rps::Scissors) | (Rps::Paper, Rps::Rock) | (Rps::Scissors, Rps::Paper) => {
            RpsWinner::Me
        }
        (Rps::Scissors, Rps::Rock) | (Rps::Rock, Rps::Paper) | (Rps::Paper, Rps::Scissors) => {
            RpsWinner::Opponent
        }
        (Rps::Scissors, Rps::Scissors) | (Rps::Rock, Rps::Rock) | (Rps::Paper, Rps::Paper) => {
            RpsWinner::Draw
        }
    }
}

impl From<&str> for RpsWinner {
    fn from(str: &str) -> Self {
        match str {
            "X" => RpsWinner::Opponent,
            "Y" => RpsWinner::Draw,
            "Z" => RpsWinner::Me,
            _ => panic!("unknown char \"{}\"", str),
        }
    }
}

fn should_pick(opponent: &Rps, winner: &RpsWinner) -> Rps {
    match (opponent, winner) {
        (Rps::Rock, RpsWinner::Draw)
        | (Rps::Paper, RpsWinner::Opponent)
        | (Rps::Scissors, RpsWinner::Me) => Rps::Rock,
        (Rps::Paper, RpsWinner::Draw)
        | (Rps::Scissors, RpsWinner::Opponent)
        | (Rps::Rock, RpsWinner::Me) => Rps::Paper,
        (Rps::Scissors, RpsWinner::Draw)
        | (Rps::Rock, RpsWinner::Opponent)
        | (Rps::Paper, RpsWinner::Me) => Rps::Scissors,
    }
}

impl From<RpsWinner> for usize {
    fn from(winner: RpsWinner) -> Self {
        match winner {
            RpsWinner::Opponent => 0,
            RpsWinner::Draw => 3,
            RpsWinner::Me => 6,
        }
    }
}

pub fn solution_1(file_contents: String) -> usize {
    file_contents
        .split("\n")
        .filter(|x| !x.is_empty())
        .map(|str| {
            let arr = str.split(' ').collect::<Vec<&str>>();
            let opponent: Rps = (*arr.get(0).unwrap()).into();
            let me: Rps = (*arr.get(1).unwrap()).into();
            let winner = check_winner(&me, &opponent);
            usize::from(winner) + usize::from(me)
        })
        .sum()
}

pub fn solution_2(file_contents: String) -> usize {
    file_contents
        .split("\n")
        .filter(|x| !x.is_empty())
        .map(|str| {
            let arr = str.split(' ').collect::<Vec<&str>>();
            let opponent: Rps = (*arr.get(0).unwrap()).into();
            let winner: RpsWinner = (*arr.get(1).unwrap()).into();
            let me = should_pick(&opponent, &winner);
            usize::from(winner) + usize::from(me)
        })
        .sum()
}

challenge_test_suite!(
    solution_1,
    15,
    13005,
    solution_2,
    12,
    11373,
    "src",
    "year_2022",
    "day_2"
);
