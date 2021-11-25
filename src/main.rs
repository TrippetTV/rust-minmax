use std::cmp::PartialEq;
use std::fmt::Debug;
use std::{fmt, thread, time};

/// When trying to read the comments to understand this code,
/// It is advised to read from the bottom up, as the most basic structures start there


fn main() {
    let mut board = Board {
        game: [[Tile::None; 3]; 3],
        trace: Trace { vector: vec![] },
        turn: Tile::X,
        stats: Stats {
            games: Trace { vector: vec![] },
            best_trace: Evaluation {
                result: 0,
                trace: Trace { vector: vec![] },
            },
            best_move: Move {
                tile: Tile::None,
                position: Position { x: 0, y: 0 },
            },
        },
    };

    minmax(&mut board, 0, 10);

    {
        let mut stats = board.clone();
        stats.stats.stats(&board);
        println!(
            "Best move is: {:#?}",
            stats.stats.best_move.to_friendly_string(),
        );
        println!("{}", stats.to_string());
        //println!("Trace:  {:#?}", stats.stats.best_trace.to_string())
    }

    thread::sleep(time::Duration::from_millis(10000))

}

///
///
/// # Arguments
///
/// * `board`: Current board state
///
/// returns: Evaluation with a result attached to it
///
/// # Examples
///
/// ```
///
/// ```
fn eval_board(board: &mut Board, depth: i32) -> Evaluation {
    let winner = eval_tile(board);
    if winner == Tile::None {
        return Evaluation {
            result: 0,
            trace: board.trace.clone(),
        };
    }

    let thing = Evaluation {
        result: if winner == Tile::X {
            i32::MAX
        } else {
            i32::MIN
        },
        trace: board.trace.clone(),
    };

    board
        .stats
        .games
        .vector
        .insert(board.stats.games.vector.len(), thing);
    return Evaluation {
        result: if winner == Tile::X {
            i32::MAX - depth
        } else {
            i32::MIN + depth
        },
        trace: board.trace.clone(),
    };
}

///
///
/// # Arguments
///
/// * `board`: The current board
///
/// returns: the Winning tile if any, otherwise Tile::None
///
/// # Examples
///
/// ```
///
/// ```
fn eval_tile(board: &Board) -> Tile {
    if board.can_continue() {
        return Tile::None;
    }
    if board.row_filled() != Tile::None {
        return board.row_filled();
    };
    if board.column_filled() != Tile::None {
        return board.column_filled();
    };
    if board.diagonal_filled() != Tile::None {
        return board.diagonal_filled();
    };

    return Tile::None;
}

///
///
/// # Arguments
///
/// * `board`: Current Board
/// * `depth`: Current Move Depth
/// * `depth_limit`: Max Move Depth
///
/// returns: Evaluation
///
/// # Examples
///
/// ```
///
/// ```
fn minmax(mut board: &mut Board, depth: i32, depth_limit: i32) -> Evaluation {
    let score = eval_board(board, depth);
    if depth >= depth_limit {
        return score;
    }

    if score.result > 0 {
        return score;
    }
    if score.result < 0 {
        return score;
    }

    let mut current = Evaluation {
        result: 0,
        trace: Trace { vector: vec![] },
    };
    // Goes through all tiles which are not played yet
    for i in 0..3 {
        for j in 0..3 {
            if board.game[i][j] == Tile::None {
                board.play(Move {
                    tile: board.turn,
                    position: Position { x: i, y: j },
                });
                current = minmax(&mut board, depth + 1, 10);
                board.undo(Move {
                    tile: board.turn,
                    position: Position { x: i, y: j },
                });
            }
        }
    }

    return current;
}
/// Stats contain all games, the best trace and the best move
#[derive(Debug, Clone)]
struct Stats {
    games: Trace<Evaluation>,
    best_trace: Evaluation,
    best_move: Move,
}
// Implement following functions for the Stats struct
impl Stats {
    /// Gets the first move in determined by self.get_best_trace()
    fn get_best_move(&self, board: &Board) -> Move {
        let mut none = 0;
        for x in 0..3 {
            for y in 0..3 {
                if board.game[x][y] == Tile::None {
                    none += 1;
                }
            }
        }
        if none == 0 {
            none = 1;
        }
        println!("Tile::None count {}", none);
        return self.best_trace.trace.vector[9 - none];
    }
    /// Saves in a field what get_best_move() returns
    fn set_best_move(&mut self, board: &Board) {
        self.best_move = self.get_best_move(board);
    }

    /// * ERROR ERROR, START OF WARNING
    /// * CURRENTLY DOESN'T WORK LIKE INTENDED, NOT VALID CODE
    /// * END OF WARNING
    fn get_best_trace(&self) -> Evaluation {
        /*
        TODO:
         Idea: get the evaluation with the best moves for each side for each move
            */

        self.games.vector.iter().max_by_key(|&y| y.result).filter(|&x| x.result != 0).unwrap().clone()
    }
    /// It sets what get_best_trace() found in a field
    fn set_best_trace(&mut self) {
        self.best_trace = self.get_best_trace().clone();
    }
    /// Gets stats about all the moves, returning a copy of itself
    fn stats(&mut self, board: &Board) -> Stats {
        /* TODO:
            change so it just returns a pretty string with all stats
            */
        self.set_best_trace();
        self.set_best_move(board);

        Stats {
            games: Trace { vector: vec![] },
            best_trace: self.best_trace.clone(),
            best_move: self.best_move.clone(),
        }
    }
}
/// The struct Board which contains all the important fields. Acts as a 'game handler' of sorts,
/// while containing all the information in one place
#[derive(Debug, Clone)]
struct Board {
    game: [[Tile; 3]; 3],
    trace: Trace<Move>,
    turn: Tile,
    stats: Stats,
}
// Implement following functions for the Board struct
impl Board {
    /// Calls self.do_move function.
    /// The reason to seperate this is because we need to call the do_move part but not any play,
    /// and to make it clear that undo is NOT playing a move, but just undoing one
    fn play(&mut self, action: Move) {
        self.do_move(action);
    }
    /// Plays the parameter move 'action' and calls opposite after the move is played
    fn do_move(&mut self, action: Move) {
        if action.tile != Tile::None {
            self.trace.vector.push(action);
        }
        self.game[action.position.x][action.position.y] = if action.tile == self.turn {
            self.turn
        } else {
            action.tile
        };
        self.turn = self.opposite()
    }
    /// Will undo the newest move done in the minmax algorithm
    fn undo(&mut self, mut action: Move) {
        self.do_move(action.replace_with(Tile::None));
        self.trace.vector.pop();
    }
    /// For every move that is done it will remember the current player by the turn field
    /// This method just returns the opposite of the tile, setting turn to the other player
    /// every time player move is done
    fn opposite(&mut self) -> Tile {
        match self.turn {
            Tile::X => Tile::O,
            Tile::O => Tile::X,
            _ => panic!("opposite of '{}' is not legal", self.turn.to_string()),
        }
    }
    /// Returns the board as a string,
    /// with info about the end winner and the current board
    fn to_string(&self) -> String {
        let mut string = String::from("");
        string += format!(
            "End result: {} \n",
            self.stats.best_trace.formatted_result()
        )
        .as_ref();
        string += format!("Current Board:\n").as_ref();
        for x in 0..3 {
            for y in 0..3 {
                string += format!("{}", self.game[x][y].to_string()).as_ref();
            }
            string += format!("\n").as_ref()
        }
        return string;
    }
    /// Returns true if the game has any remaining moves, indicated by Tile::None being on the board
    /// otherwise return false
    fn can_continue(&self) -> bool {
        for row in self.game.iter() {
            for &tile in row.iter() {
                if tile == Tile::None {
                    return true;
                }
            }
        }
        return false;
    }

    fn row_filled(&self) -> Tile {
        for &row in self.game.iter() {
            if row[0] == row[1] && row[1] == row[2] {
                return row[1];
            }
        }
        return Tile::None;
    }

    fn column_filled(&self) -> Tile {
        if (0..self.game.len()).any(|i| self.game.iter().all(|row| row[i] == self.turn)) {
            return self.turn;
        }

        return Tile::None;
    }
    /// Determines if a diagonal of the current board is filled, and what Tile it is filled with
    fn diagonal_filled(&self) -> Tile {
        if (self.game[0][0] == self.game[1][1] && self.game[1][1] == self.game[2][2])
            || (self.game[2][0] == self.game[1][1] && self.game[1][1] == self.game[0][2])
        {
            return self.game[1][1];
        }
        return Tile::None;
    }
}
/// An Evaluation needs to contain the end result if any to the current board,
/// and the actual moves on that board
#[derive(Debug, Clone)]
struct Evaluation {
    result: i32,
    trace: Trace<Move>,
}

// Implement following functions for the Evaluation struct
impl Evaluation {
    /// Returns a string reference which shows the end result of the current Evaluation trace
    fn formatted_result(&self) -> &str {
        match self.winner_tile() {
            Tile::O => "O Win",
            Tile::X => "X Win",
            _ => "Draw",
        }
    }
    /// Determines if a winner has been established or not
    fn winner_tile(&self) -> Tile {
        match self.result {
            i32::MIN..=-1 => Tile::O,
            1..=i32::MAX => Tile::X,
            _ => Tile::None,
        }
    }
    /// Returns all moves in the current array
    fn to_string(&self) -> String {
        return format!("{:#?}", self.trace.vector.as_slice().iter());
    }
}

/// <T> means Generic Type, thus we can create a Trace<T> of any type, for example Trace<Move>.
/// When creating this Trace it will remember the type T was used for and create the Vec<T> with
/// the same type as the trace
/// Vector being an array
#[derive(Debug, Clone)]
struct Trace<T> {
    vector: Vec<T>,
}

/// Each Move contains two different pieces of information, being the tile and position
#[derive(Debug, Copy, Clone, PartialEq)]
struct Move {
    tile: Tile,
    position: Position,
}

/// Implement following functions for the Move struct
impl Move {
    /// Used to "undo" a move.
    fn replace_with(&mut self, tile: Tile) -> Move {
        Move {
            tile,
            position: self.position,
        }
    }
    /// If the current move is Tile::None, meaning no player,
    /// it will crash the code by design,
    /// otherwise return the move as a string for debugging/info
    fn to_friendly_string(&self) -> String {
        match self.tile {
            // 'panic!' will crash the program with the following error message
            Tile::None => panic!(
                "Invalid move as: {} , Position: {}",
                self.tile.to_string(),
                self.position.to_string()
            ),
            // 'format!' returns a String object, and it is used to send the following information
            _ => format!(
                "Player: {}, Position: {}",
                self.tile.to_string(),
                self.position.to_string()
            ),
        }
    }
}
/// A struct which contain coordinates as the fields
#[derive(Debug, Copy, Clone, PartialEq)]
struct Position {
    x: usize,
    y: usize,
}
/// Implement following functions for the Position struct
impl Position {
    // Returns its own coordinates
    fn to_string(&self) -> String {
        return format!("X = {}, Y = {}", self.x, self.y);
    }
}

/// Tile is used as a state for each piece on the board
#[derive(Debug, Copy, Clone, PartialEq)]
enum Tile {
    None,
    O,
    X,
}
/// Implement following functions for the Tile enum
impl Tile {
    // This will return a character based on what enum state self is
    fn to_string(&self) -> String {
        String::from(match self {
            Tile::O => "O",
            Tile::X => "X",
            Tile::None => "_",
        })
    }
}
