use std::str::FromStr;

#[derive(Copy, Clone)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    fn from_opponent_char(input: &str) -> Result<Self, &'static str> {
        match input {
            "A" => Ok(Move::Rock),
            "B" => Ok(Move::Paper),
            "C" => Ok(Move::Scissors),
            _ => Err("Unrecognized {input} move"),
        }
    }

    fn from_player_char(input: &str) -> Result<Self, &'static str> {
        match input {
            "X" => Ok(Move::Rock),
            "Y" => Ok(Move::Paper),
            "Z" => Ok(Move::Scissors),
            _ => Err("Unrecognized {input} move"),
        }
    }

    fn score(&self) -> u64 {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }

    fn for_outcome(&self, outcome: Outcome) -> Move {
        use Move::*;
        use Outcome::*;

        match (self, outcome) {
            (x, Draw) => *x,
            (Rock, Win) => Paper,
            (Paper, Win) => Scissors,
            (Scissors, Win) => Rock,
            (Rock, Lose) => Scissors,
            (Paper, Lose) => Rock,
            (Scissors, Lose) => Paper,
        }
    }
}

enum Outcome {
    Win,
    Lose,
    Draw,
}

impl FromStr for Outcome {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Outcome::Lose),
            "Y" => Ok(Outcome::Draw),
            "Z" => Ok(Outcome::Win),
            _ => Err("Invalid char {c}".to_string()),
        }
    }
}

impl Outcome {
    fn score(&self) -> u64 {
        match self {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Lose => 0,
        }
    }
}

struct Match {
    oponnent: Move,
    player: Move,
}

impl FromStr for Match {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((oponnent, player)) = s.split_once(' ') {
            return Ok(Match {
                oponnent: Move::from_opponent_char(oponnent).unwrap(),
                player: Move::from_player_char(player).unwrap(),
            });
        }

        Err("Cannot parse line {line}".to_string())
    }
}

impl Match {
    fn from_line_with_outcome(s: &str) -> Result<Self, &'static str> {
        if let Some((oponnent, player)) = s.split_once(' ') {
            let opponent_move = Move::from_opponent_char(oponnent).unwrap();

            return Ok(Match {
                oponnent: opponent_move,
                player: Move::for_outcome(&opponent_move, player.parse::<Outcome>().unwrap()),
            });
        }

        Err("Cannot parse line {line}")
    }

    fn outcome(&self) -> Outcome {
        use Move::*;
        use Outcome::*;

        match self {
            Match {
                oponnent: Rock,
                player: Rock,
            } => Draw,
            Match {
                oponnent: Paper,
                player: Paper,
            } => Draw,
            Match {
                oponnent: Scissors,
                player: Scissors,
            } => Draw,
            Match {
                oponnent: Rock,
                player: Paper,
            } => Win,
            Match {
                oponnent: Paper,
                player: Rock,
            } => Lose,
            Match {
                oponnent: Scissors,
                player: Rock,
            } => Win,
            Match {
                oponnent: Rock,
                player: Scissors,
            } => Lose,
            Match {
                oponnent: Paper,
                player: Scissors,
            } => Win,
            Match {
                oponnent: Scissors,
                player: Paper,
            } => Lose,
        }
    }
}

pub fn part1(input: &str) -> u64 {
    input
        .lines()
        .map(|line| {
            let m = line.parse::<Match>().unwrap();
            m.outcome().score() + m.player.score()
        })
        .sum()
}

pub fn part2(input: &str) -> u64 {
    input
        .lines()
        .map(|line| {
            let m = Match::from_line_with_outcome(line).unwrap();
            m.outcome().score() + m.player.score()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = "A Y
B X
C Z";

    #[test]
    fn part_1_works() {
        let result = part1(INPUT);
        assert_eq!(result, 15);
    }

    #[test]
    fn part_2_works() {
        let result = part2(INPUT);
        assert_eq!(result, 12);
    }
}
