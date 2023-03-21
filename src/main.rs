use std::{fmt, io};

enum Error {
    Board(String),
}

#[derive(PartialEq, Clone, Copy)]
enum PlayerMovement {
    X,
    O,

    Empty,
}

impl fmt::Display for PlayerMovement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::X => 'X',
                Self::O => 'O',
                Self::Empty => '-',
            }
        )
    }
}

struct Board {
    rows: [[PlayerMovement; 3]; 3],
    next: PlayerMovement,
}

impl Board {
    fn put(&mut self, movement: PlayerMovement, position: Position) -> Result<(), Error> {
        let column = position.0;
        let row = position.1;

        match self.rows[row][column] {
            PlayerMovement::Empty => {
                self.rows[row][column] = movement;

                Ok(())
            }
            _ => Err(Error::Board("Position is not empty!".into())),
        }
    }

    fn swap_turns(&mut self) {
        self.next = match self.next {
            PlayerMovement::X => PlayerMovement::O,
            _ => PlayerMovement::X,
        }
    }
}

impl Default for Board {
    fn default() -> Self {
        use PlayerMovement::*;

        Self {
            rows: [
                [Empty, Empty, Empty],
                [Empty, Empty, Empty],
                [Empty, Empty, Empty],
            ],
            next: X,
        }
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, " [a|b|c]")?;

        for (idx, row) in self.rows.iter().enumerate() {
            let row = row.map(|m| m.to_string()).join("|");

            writeln!(f, "{}[{}]", idx + 1, row)?;
        }

        Ok(())
    }
}

struct Position(usize, usize);

impl TryFrom<&str> for Position {
    type Error = Error;

    fn try_from(value: &str) -> Result<Position, Error> {
        match value {
            "a1" => Ok(Position(0, 0)),
            "a2" => Ok(Position(0, 1)),
            "a3" => Ok(Position(0, 2)),
            "b1" => Ok(Position(1, 0)),
            "b2" => Ok(Position(1, 1)),
            "b3" => Ok(Position(1, 2)),
            "c1" => Ok(Position(2, 0)),
            "c2" => Ok(Position(2, 1)),
            "c3" => Ok(Position(2, 2)),
            _ => Err(Self::Error::Board("Invalid position! try again.".into())),
        }
    }
}

fn main() {
    let mut board = Board::default();
    let stdin = io::stdin();

    println!("Welcome to Tic Tac Toe! Place X or O using movements like (b2, c3, a2), Good luck!");

    loop {
        println!("{}", board);
        println!("It's {}'s turn! [Enter your movement]: ", &board.next);
        print!("{}[2J", 27 as char);

        let mut position = String::new();

        stdin.read_line(&mut position).expect("Coudln't read line.");

        let position = match Position::try_from(position.as_str().trim()) {
            Err(Error::Board(e)) => {
                println!("{}", e);
                continue;
            }
            Ok(position) => position,
        };

        if let Err(Error::Board(e)) = board.put(board.next, position) {
            println!("{}", e);
            continue;
        }

        board.swap_turns();
    }
}
