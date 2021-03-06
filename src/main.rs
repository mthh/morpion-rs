
extern crate colored;
extern crate rand;
use std::io;
use std::io::Write;
use colored::*;
use rand::seq::sample_iter;


macro_rules! check_state {
    ($jeu:expr) => {{
        match $jeu.check_state() {
            State::Win(p) => {
                print!("{}[2J", 27 as char);
                println!("    {:?} wins !!\n\n{}", p, $jeu);
                break;
            },
            State::Tie => {
                print!("{}[2J", 27 as char);
                println!("    Tie !!\n\n{}", $jeu);
                break;
            },
            _ => (),
        }
    }}
}

macro_rules! print_err_continue_loop {
    ($err:expr) => {{
        println!("{}", $err);
        continue;
    }}
}

#[derive(Clone)]
enum Command {
    Quit,
    Val(usize)
}

#[derive(Debug)]
struct IndexScore(Option<usize>, i32);

#[derive(Debug)]
struct Jeu {
    board: [char; 9],
    last_player: Option<Player>
}

#[derive(Clone, Debug, PartialEq)]
enum Player { X, O }

enum State {
    Win(Player),
    Tie,
    Continue,
}

impl<'a> Into<char> for &'a Player {
    fn into(self) -> char {
        match self {
            Player::X => 'X',
            Player::O => 'O'
        }
    }
}

fn format_char_color(c: char) -> String {
    match c {
        'X' => format!(" {} ", c.to_string().blue()),
        'O' => format!(" {} ", c.to_string().red()),
        _ => unreachable!(),
    }
}

impl std::fmt::Display for Jeu {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f, "{} | {} | {}\n---------------\n{} | {} | {}\n---------------\n{} | {} | {}\n",
            if self.board[0] == '.' { format!("{}", "[1]".yellow()) } else { format_char_color(self.board[0]) },
            if self.board[1] == '.' { format!("{}", "[2]".yellow()) } else { format_char_color(self.board[1]) },
            if self.board[2] == '.' { format!("{}", "[3]".yellow()) } else { format_char_color(self.board[2]) },
            if self.board[3] == '.' { format!("{}", "[4]".yellow()) } else { format_char_color(self.board[3]) },
            if self.board[4] == '.' { format!("{}", "[5]".yellow()) } else { format_char_color(self.board[4]) },
            if self.board[5] == '.' { format!("{}", "[6]".yellow()) } else { format_char_color(self.board[5]) },
            if self.board[6] == '.' { format!("{}", "[7]".yellow()) } else { format_char_color(self.board[6]) },
            if self.board[7] == '.' { format!("{}", "[8]".yellow()) } else { format_char_color(self.board[7]) },
            if self.board[8] == '.' { format!("{}", "[9]".yellow()) } else { format_char_color(self.board[8]) },
        )
    }
}

impl Jeu {
    pub fn new() -> Self {
        Jeu {
            board: ['.', '.', '.', '.', '.', '.', '.', '.', '.'],
            last_player: None,
        }
    }

    pub fn empty_indexes(&self) -> Vec<usize> {
        self.board.iter()
            .enumerate()
            .filter_map(|(i, c)| match *c { '.' =>  Some(i), _ => None })
            .collect()
    }

    pub fn tour_joueur(&self) -> Player {
        let (cx, co) = self.board.iter()
            .fold((0, 0), |(sum_x, sum_o), c| {
                match *c {
                    'X' => (sum_x + 1, sum_o),
                    'O' => (sum_x, sum_o + 1),
                    _ => (sum_x, sum_o)
                }
            });
        if cx <= co { Player::X } else { Player::O }
    }

    pub fn make_move(&mut self, player: &Player, position: usize) -> Result<(), String> {
        if self.board[position] != '.' {
            return Err("Invalid move ! Please try again !".to_string());
        }
        if let Some(ref p) = self.last_player {
            if p == player {
                return Err("Invalid player ! Please try again !".to_string());
            }
        };
        self.last_player = Some(player.clone());
        self.board[position] = match player {
            Player::X => 'X',
            Player::O => 'O',
        };
        Ok(())
    }

    pub fn check_state(&self) -> State {
        if wins(&self, &Player::X) {
            State::Win(Player::X)
        } else if wins(&self, &Player::O) {
            State::Win(Player::O)
        } else if self.empty_indexes().is_empty() {
            State::Tie
        } else {
            State::Continue
        }
    }
}

fn min_max(board: &mut Jeu, player: &Player, ai_player: &Player) -> IndexScore {
    let available_spots = board.empty_indexes();
    let other_player = match ai_player {
        Player::X => Player::O,
        _ => Player::X,
    };
    // unsafe { FC += 1; }
    if wins(board, &other_player) {
        IndexScore(None, -10)
    } else if wins(board, ai_player) {
        IndexScore(None, 10)
  	} else if available_spots.is_empty() {
        IndexScore(None, 0)
    } else if available_spots.len() == 9 && player == ai_player {
        IndexScore(Some(4), 100)
    } else {
        let mut moves = Vec::with_capacity(available_spots.len());
        for ix in available_spots {
            let mut mvt = IndexScore(Some(ix), 0);
            board.board[ix] = player.into();
            if player == ai_player {
              let result = min_max(board, &other_player, ai_player);
              mvt.1 = result.1;
            } else {
              let result = min_max(board, ai_player, ai_player);
              mvt.1 = result.1;
            }
            board.board[ix] = '.';
            moves.push(mvt);
        }
        let mut best_move = None;
        if player == ai_player {
            let mut best_score = -10000;
            for (i, m) in moves.iter().enumerate() {
                if m.1 > best_score {
                    best_score = m.1;
                    best_move = Some(i);
                }
            }
        } else {
            let mut best_score = 10000;
            for (i, m) in moves.iter().enumerate() {
                if m.1 < best_score {
                    best_score = m.1;
                    best_move = Some(i);
                }
            }
        }
        match best_move {
            Some(b) => moves.remove(b),
            None => panic!("No best move found!!")
        }
    }
}

fn wins(board: &Jeu, p: &Player) -> bool {
    let player: char = p.into();
    ((board.board[0] == player && board.board[1] == player && board.board[2] == player) ||
        (board.board[3] == player && board.board[4] == player && board.board[5] == player) ||
        (board.board[6] == player && board.board[7] == player && board.board[8] == player) ||
        (board.board[0] == player && board.board[3] == player && board.board[6] == player) ||
        (board.board[1] == player && board.board[4] == player && board.board[7] == player) ||
        (board.board[2] == player && board.board[5] == player && board.board[8] == player) ||
        (board.board[0] == player && board.board[4] == player && board.board[8] == player) ||
        (board.board[2] == player && board.board[4] == player && board.board[6] == player))
}

fn read_command() -> Result<Command, String> {
    let mut input = String::new();
    match std::io::stdin().read_line(&mut input) {
        Ok(_) => {
            let trimmed_input: &str = input.trim();
            if trimmed_input == "q" {
                Ok(Command::Quit)
            } else {
                match input.trim().parse() {
                    Ok(parsed_input) => Ok(Command::Val(parsed_input)),
                    Err(a) => Err(a.to_string())
                }
            }
        },
        Err(err) => Err(err.to_string())
    }
}

// static mut FC: i32 = 0;

fn main() {
    let mut rng = rand::thread_rng();
    let mut jeu = Jeu::new();
    let starting_player = sample_iter(&mut rng, vec!['X', 'O'], 1).unwrap()[0];
    print!("{}[2J", 27 as char);
    println!("You are : O\nComputer is : X\nPlayer to start : {} \n", starting_player);
    println!("Instruction : Enter the index of the case to use (or q to quit)\n");

    if starting_player == 'X' {
        let first_move = min_max(&mut jeu, &Player::X, &Player::X).0.unwrap();
        jeu.make_move(&Player::X, first_move).unwrap();
    }

    loop {
        print!("\n{}\nO > ", jeu);
        io::stdout().flush().unwrap();
        match read_command() {
            Ok(Command::Val(index)) => if 1 <= index && index <= 9 {
                if let Err(err) = jeu.make_move(&Player::O, index - 1) {
                    print_err_continue_loop!(err)
                }
            } else {
                print_err_continue_loop!("Incorrect index. Please try again");
            },
            Ok(Command::Quit) => break,
            Err(err) => print_err_continue_loop!(err),
        };

        check_state!(&jeu);

        let to_be_played = match min_max(&mut jeu, &Player::X, &Player::X).0 {
            Some(ix) => ix,
            None => sample_iter(&mut rng, jeu.empty_indexes(), 1).unwrap()[0]
        };
        print!("{}[2J", 27 as char);
        // unsafe { println!("FC : {}", FC); FC = 0; }
        jeu.make_move(&Player::X, to_be_played).unwrap();
        check_state!(&jeu);
    }
}
