use either::{Either, Left, Right};
use std::marker::PhantomData;
use tyrade::*;

#[derive(Debug)]
pub enum GameError {
    SpotTaken,
}

pub type Result<T> = std::result::Result<T, GameError>;

tyrade! {
    enum TPlayer {
        PlayerX,
        PlayerO
    }

    fn TNextPlayer<P>() {
        match P {
            PlayerX => PlayerO,
            PlayerO => PlayerX,
        }
    }
}

impl Clone for PlayerO {
    fn clone(&self) -> Self {
        PlayerO(PhantomData)
    }
}
impl Copy for PlayerO {}
impl Clone for PlayerX {
    fn clone(&self) -> Self {
        PlayerX(PhantomData)
    }
}
impl Copy for PlayerX {}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Player {
    X,
    O,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Spot {
    Player(Player),
    Empty,
}
pub trait Spotted {
    fn spot() -> Spot;
}
impl Spotted for PlayerX {
    fn spot() -> Spot {
        Spot::Player(Player::X)
    }
}
impl Spotted for PlayerO {
    fn spot() -> Spot {
        Spot::Player(Player::O)
    }
}

pub type Board = [[Spot; 3]; 3];

#[derive(Debug, Clone, Copy)]
pub struct State<Player> {
    pub board: Board,
    _player_marker: PhantomData<Player>,
}

impl Into<Either<State<PlayerO>, State<PlayerX>>> for State<PlayerX> {
    fn into(self) -> Either<State<PlayerO>, State<PlayerX>> {
        Right(self)
    }
}

impl Into<Either<State<PlayerO>, State<PlayerX>>> for State<PlayerO> {
    fn into(self) -> Either<State<PlayerO>, State<PlayerX>> {
        Left(self)
    }
}

impl Default for State<PlayerO> {
    fn default() -> Self {
        State::<PlayerO> {
            board: [[Spot::Empty; 3]; 3],
            _player_marker: PhantomData,
        }
    }
}

impl<Player: ComputeTNextPlayer + Spotted> State<Player> {
    pub fn next(self, pos: (usize, usize)) -> Result<State<TNextPlayer<Player>>> {
        Ok(State::<TNextPlayer<Player>> {
            board: update_board::<Player>(self.board, pos)?,
            _player_marker: PhantomData,
        })
    }
}

fn update_board<Player: Spotted>(mut board: Board, pos: (usize, usize)) -> Result<Board> {
    if pos.0 > 2 || pos.1 > 2 {
        panic!("invalid position {pos:?}")
    } else if board[pos.0][pos.1] != Spot::Empty {
        Err(GameError::SpotTaken)
    } else {
        board[pos.0][pos.1] = Player::spot();
        Ok(board)
    }
}

fn check_winner(board: &Board) -> Option<Player> {
    [
        ((0, 0), (0, 1), (0, 2)),
        ((1, 0), (1, 1), (1, 2)),
        ((2, 0), (2, 1), (2, 2)),
        ((0, 0), (1, 0), (2, 0)),
        ((0, 1), (1, 1), (2, 1)),
        ((0, 2), (1, 2), (2, 2)),
        ((0, 0), (1, 1), (2, 2)),
        ((0, 2), (1, 1), (2, 0)),
    ]
    .iter()
    .fold(None, |current, row| {
        if current.is_some() {
            current
        } else if board[row.0 .0][row.0 .1] == board[row.1 .0][row.1 .1]
            && board[row.1 .0][row.1 .1] == board[row.2 .0][row.2 .1]
        {
            Some(board[row.0 .0][row.0 .1])
        } else {
            None
        }
    })
    .and_then(|val| match val {
        Spot::Player(p) => Some(p),
        Spot::Empty => None,
    })
}
