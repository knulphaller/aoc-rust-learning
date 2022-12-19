use std::fs::read_to_string;
use std::str::FromStr;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let score: usize = read_to_string("src/input.txt")
        .unwrap()
        .lines()
        .map(Round::from_str)
        .map(|s| s.unwrap().ourz())
        .sum();

    println!("Score: {score:?}");
    Ok(())
}

#[derive(Debug, Clone, Copy)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, Clone, Copy)]
struct Round {
    theirs: Move,
    ours: Move,
}

#[derive(Debug)]
enum Outcome {
    Win,
    Lose,
    Draw,
}

impl Move {
    const ALL_MOVES: [Move; 3] = [Move::Rock, Move::Paper, Move::Scissors];

    fn winning_move(self) -> Self {
        Self::ALL_MOVES
            .iter()
            .copied()
            .find(|m| m.beats(self))
            .expect("at least one move beats us")
    }

    fn losing_move(self) -> Self {
        Self::ALL_MOVES
            .iter()
            .copied()
            .find(|&m| self.beats(m))
            .expect("At least one move beats us")
    }

    fn drawing_move(self) -> Self {
        self
    }

    fn points(self) -> usize {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }

    fn beats(self, other: Move) -> bool {
        matches!(
            (self, other),
            (Self::Rock, Self::Scissors)
                | (Self::Paper, Self::Rock)
                | (Self::Scissors, Self::Paper)
        )
    }

    fn outcomez(self, theirs: Move) -> Outcome {
        if self.beats(theirs) {
            Outcome::Win
        } else if theirs.beats(self) {
            Outcome::Lose
        } else {
            Outcome::Draw
        }
    }
}

impl Round {
    fn outcome(self) -> Outcome {
        self.ours.outcomez(self.theirs)
    }

    fn ourz(self) -> usize {
        self.ours.points() + self.outcome().points()
    }
}

impl Outcome {
    fn points(self) -> usize {
        match self {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Lose => 0,
        }
    }

    fn matching_move(self, theirs: Move) -> Move {
        match self {
            Outcome::Win => theirs.winning_move(),
            Outcome::Lose => theirs.losing_move(),
            Outcome::Draw => theirs.drawing_move(),
        }
    }
}

impl FromStr for Round {
    type Err = color_eyre::Report;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        let (Some(theirs), Some(' '), Some(ours), None) = (chars.next(), chars.next(), chars.next(), chars.next()) else {
            return Err(color_eyre::eyre::eyre!("Expected <theirs>SP<ours>EOF, got {s:?}"));
        };

        let theirs = Move::try_from(theirs)?;
        let outcome = Outcome::try_from(ours)?;
        let ours = outcome.matching_move(theirs);

        Ok(Self { theirs, ours })
    }
}

impl TryFrom<char> for Move {
    type Error = color_eyre::Report;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' => Ok(Move::Rock),
            'B' => Ok(Move::Paper),
            'C' => Ok(Move::Scissors),
            _ => Err(color_eyre::eyre::eyre!("Not a valid move: {value:?}")),
        }
    }
}

impl TryFrom<char> for Outcome {
    type Error = color_eyre::Report;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'X' => Ok(Outcome::Lose),
            'Y' => Ok(Outcome::Draw),
            'Z' => Ok(Outcome::Win),
            _ => Err(color_eyre::eyre::eyre!("Not a valid outcome: {value:?}")),
        }
    }
}
