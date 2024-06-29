#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_assignments)]

use std::iter::Filter;
use std::*;
use std::f32::consts::E;
use std::thread::sleep;
use colored::*;
use rand::seq::index;
use std::env;
use std::fs;
use std::io;
use std::time::{Duration, Instant};

use rand::seq::SliceRandom;
use rand::thread_rng;
use rand::Rng;
use rand::distributions::{Distribution, Uniform};


const DISPLAY_GRID:   bool = true;
const RENDER_PIECES:  bool = true;


const NONE:      u8 = 0;
const KING:      u8 = 1;
const PAWN:      u8 = 2;
const KNIGHT:    u8 = 3;
const BISHOP:    u8 = 4;
const ROOK:      u8 = 5;
const QUEEN:     u8 = 6;

const WHITE:     u8 = 8;
const BLACK:     u8 = 16;

const TYPEMASK:  u8 = 0b00111;
const COLORMASK: u8 = WHITE | BLACK;

const SETUP: &str = "8/8/8/8/1k5Q/8/8/8 b - - 0 1";
//const SETUP: &str = "8/8/8/8/4n1n1/2n3n1/P1P1P1P1/8 w - - 0 1";
//const SETUP: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQkq - 0 1";
//const SETUP: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w - - 0 1";


const DIRECTION_OFFSETS: [i8; 8] = [ 8, 1, -8, -1, 9, -7, -9, 7 ];

#[derive(Clone, Copy, Debug)]
struct Piece {
    id: u8,
    index: u8,
}

impl Piece {
    fn is_color(&self, color: u8) -> bool {
        return (self.id & COLORMASK) == color;
    }

    fn color(&self) -> u8 {
        return self.id & COLORMASK;
    }

    fn piece_type(&self) -> u8{
        return self.id & TYPEMASK;
    }

    fn is_rook_or_queen(&self) -> bool {
        return (self.id & 0b110) == 0b110;
    }

    fn is_bishop_or_queen(&self) -> bool {
        return (self.id & 0b101) == 0b101;
    }

    fn is_sliding_piece(&self) -> bool {
        return (self.id & 0b100) != 0;
    }

    fn enemy_color(&self) -> u8 {
        return if self.color() == WHITE { BLACK } else { WHITE }
    }
    fn has_trait(&self, thing: u8) -> bool {
        if thing == 255 {
            return true;
        } else if thing == WHITE || thing == BLACK {
            return self.id & COLORMASK == thing;
        } else if thing < WHITE {
            return self.id & TYPEMASK == thing;
        } else {
            return self.id == thing;
        }
    }
    

    fn to_char(&self) -> char {
        let tipe = match self.piece_type() {
            KING => 'k',
            PAWN => 'p',
            KNIGHT => 'n',
            BISHOP => 'b',
            ROOK => 'r',
            QUEEN => 'q',
            0 => '-',
            _ => panic!("ur mum")
        };

        return if self.color() == WHITE { tipe.to_ascii_uppercase() } else { tipe }
    }

    fn to_pretty_char(&self) -> ColoredString {
        if RENDER_PIECES {
            return match self.color() {
                WHITE => match self.piece_type() {
                    KING => "\u{2654}".bright_yellow().bold(),
                    PAWN => "\u{2659}".bright_yellow().bold(),
                    KNIGHT => "\u{2658}".bright_yellow().bold(),
                    BISHOP => "\u{2657}".bright_yellow().bold(),
                    ROOK => "\u{2656}".bright_yellow().bold(),
                    QUEEN => "\u{2655}".bright_yellow().bold(),
                    _ => panic!("{} is an invalid white piece type", self.piece_type())
                }
                BLACK => match self.piece_type() {
                    KING => "\u{2654}".cyan().bold(),
                    PAWN => "\u{2659}".cyan().bold(),
                    KNIGHT => "\u{2658}".cyan().bold(),
                    BISHOP => "\u{2657}".cyan().bold(),
                    ROOK => "\u{2656}".cyan().bold(),
                    QUEEN => "\u{2655}".cyan().bold(),
                    _ => panic!("{} is an invalid white piece type", self.piece_type())
                }

                0 => "\u{2007}".black(),
                _ => panic!("{} is an invalid color", self.color())
            }
        } else {
            return match self.color() {
                WHITE => match self.piece_type() {
                            KING => "K".red(),
                            PAWN => "P".red(),
                            KNIGHT => "N".red(),
                            BISHOP => "B".red(),
                            ROOK => "R".red(),
                            QUEEN => "Q".red(),
                            _ => panic!("{} is an invalid white piece type", self.piece_type())
                        }
                BLACK => match self.piece_type() {
                            KING => "k".blue(),
                            PAWN => "p".blue(),
                            KNIGHT => "n".blue(),
                            BISHOP => "b".blue(),
                            ROOK => "r".blue(),
                            QUEEN => "q".blue(),
                            _ => panic!("{} is an invalid black piece type", self.piece_type())
                        }
                0 => "\u{2007}".black(),
                _ => panic!("{} is an invalid color", self.color())
            }
        }
        
    }
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_pretty_char())
    }
}

#[derive(Debug, Clone, Copy)]
struct Board {
    squares: [ Piece; 64 ],
    turn_index: u32,
    enpassantables: [[ bool; 8 ]; 2],
}

impl Board {
    fn color_to_move(&self) -> u8 {
        return if self.turn_index % 2 == 0 { WHITE } else { BLACK };
    }

    fn piece_grid(&self, cond: u8)/* -> [ Piece; 64 ] */ {
        let new_board: [ Piece; 64 ] = [ Piece { id: 0, index: 0 }; 64];
    }

    fn display(&self) {
        display_board(&self, 255);
    }

    fn display_with_thing(&self, thing: u8) {
        display_board(&self, thing);
    }

    fn make(&mut self, movee: &Move) {
        self.enpassantables[1] = self.enpassantables[0];

        if self.squares[movee.start as usize].piece_type() == PAWN && movee.start / 8 == 1 && movee.end / 8 == 3 {
            self.enpassantables[0][(movee.start % 8) as usize] = false;
        }

        self.squares[movee.end as usize].id = NONE;
        self.squares.swap(movee.start as usize, movee.end as usize);

        self.squares[movee.start as usize].index = movee.start;
        self.squares[movee.end as usize].index = movee.end;

        self.turn_index += 1;

    }

    fn unmake(&mut self, movee: &Move) {
        self.squares.swap(movee.start as usize, movee.end as usize);
        self.turn_index -= 1;
    }

    fn gen_sliding_moves(&self, piece: &Piece) -> Vec<Move> {
        let start_pos: u8 = piece.index;
        let mut end_pos;
        let mut moves = vec![];
        let s_index: usize;
        let e_index: usize;

        match piece.piece_type() {
            ROOK => { s_index = 0; e_index = 4; },
            BISHOP => { s_index = 4; e_index = 8; },
            QUEEN => { s_index = 0; e_index = 8; },
            _ => panic!("impossible! what {}", piece.to_char()),
        }

        for i in s_index..e_index {
            end_pos = start_pos;
            for j in 0..precompute_move_data()[start_pos as usize][i] {

                

                end_pos = (end_pos as i8 + DIRECTION_OFFSETS[i] as i8) as u8;
                let target_piece: Piece = self.squares[end_pos as usize];

                if piece.is_color(target_piece.color()) {
                    break;
                }

                moves.push(Move { start: start_pos, end: end_pos});

                if target_piece.is_color(piece.enemy_color()) {
                    break;
                }
            }
        }
        return moves;
    }

    fn gen_other_moves(&self, piece: &Piece) -> Vec<Move> {
        let mut moves: Vec<Move> = vec![];

        let mut unchecked_indices: Vec<(i8,i8)> = vec![];
        
        match piece.piece_type() {
            PAWN => {
                if piece.index / 8 == 0 || piece.index / 8 == 7 {
                    return moves;
                }

                match piece.color() {
                    WHITE => {
                        unchecked_indices.push((0,1));
                        if piece.index / 8 == 1 { unchecked_indices.push((0,2)) }

                        if self.squares[(piece.index + 7) as usize].color() == piece.enemy_color() {
                            unchecked_indices.push((-1,1));
                        }
                        if self.squares[(piece.index + 9) as usize].color() == piece.enemy_color() {
                            unchecked_indices.push((1,1));
                        }
                    },
                    BLACK => {
                        unchecked_indices.push((0,-1));
                        if piece.index / 8 == 6 { unchecked_indices.push((0,-2)) }

                        if self.squares[(piece.index - 7) as usize].color() == piece.enemy_color() {
                            unchecked_indices.push((1,-1));
                        }
                        if self.squares[(piece.index - 9) as usize].color() == piece.enemy_color() {
                            unchecked_indices.push((-1,-1));
                        }
                    },
                    _ => panic!("shouldnt be {}", piece),
                }
            }
            KNIGHT => {
                unchecked_indices = [(1, 2), (2, 1), (2, -1), (1, -2), (-1, -2), (-2, -1), (-2, 1), (-1, 2)].to_vec();
            },
            KING => {
                unchecked_indices = [(0, 1), (1, 1), (1, 0), (1, -1), (0, -1), (-1, -1), (-1, 0), (-1, 1)].to_vec();
            }
            _ => panic!("cant move nothing {}", piece.to_char()),
        }
        
        let mut alarm: bool = false;

        for (i, m) in unchecked_indices.iter().enumerate() {
            if alarm && piece.piece_type() == PAWN { alarm = false; continue }

            let x: i8 = (piece.index % 8) as i8;
            let y: i8 = (piece.index / 8) as i8;

            let target_pos: (i8, i8) = ( x + m.0, y + m.1 );

            if target_pos.0 < 0 || target_pos.0 > 7 || target_pos.1 < 0 || target_pos.1 > 7 {
                continue;
            }
            

            let target_piece: Piece = self.squares[ (8 * target_pos.1 + target_pos.0) as usize ];

            if target_piece.is_color(piece.color()) {
                continue;
            }

            if piece.piece_type() == PAWN && i == 0 && target_piece.piece_type() != NONE {
                alarm = true;
                continue;
            }

            if (y == 1 || y == 6) && piece.piece_type() == PAWN && i == 1 && target_piece.piece_type() != NONE {
                continue;
            }
            
            moves.push(Move { start: piece.index as u8, end: target_piece.index as u8});
        }

        return moves;
    }

    //let mut moves: Vec<Move> = vec![];
    fn gen_moves(&self) -> Vec<Move> {
        let mut moves: Vec<Move> = vec![];
        let mut piece: Piece;

        for piece in &self.squares {
            if piece.piece_type() == NONE || piece.color() != self.color_to_move() {
                continue;
            }

            if piece.is_sliding_piece() {
                moves.append(&mut self.gen_sliding_moves(piece));
            } else {
                moves.append(&mut self.gen_other_moves(piece));
            }


        }

        return moves;
    }

    fn gen_moves_at(&self, index: u8) -> Vec<Move> {
        let mut moves: Vec<Move> = vec![];
        let piece = self.squares[index as usize];

        if piece.is_sliding_piece() {
            moves.append(&mut self.gen_sliding_moves(&piece));
        } else {
            moves.append(&mut self.gen_other_moves(&piece));
        }

        return moves;
    }

    fn check_legality(&self, movee: &Move) -> bool {
        let mut temp = self.clone();

        temp.make(movee);
        let moves_to_check = temp.gen_moves();

        for i in moves_to_check {
            if temp.squares[i.end as usize].piece_type() == KING {
                return false;
            }
        }

        return true;
    }

    fn display_all_moves(&self) {
        let moves = self.gen_moves();
        let mut boards = vec![ self.to_owned(); moves.len() ];

        for i in 0..moves.len() {
            boards[i].make(&moves[i]);
            boards[i].display();
        }
    }

}

fn next_layer(layer: &Vec<Board>) -> Vec<Board> {
    let mut moves = vec![];
    let mut next_layer = vec![];

    for (pos, board) in layer.iter().enumerate() {
        let mut t = board.gen_moves();
        let mut t1 = vec![ layer[pos]; t.len() ];
        moves.append(&mut t);
        next_layer.append(&mut t1);
    }

    for i in 0..moves.len() {
        next_layer[i].make(&moves[i]);
    }

    return next_layer;
}

fn parse_move(input: &str) -> Move {
    let mut i = &input[..input.len() - 2];

    if i.len() != 4 {
        panic!("Input should be length 4, {}", input);
    }

    let nums: Vec<u32> = i.chars().filter(|x| x.is_ascii_digit()).map(|x| x.to_digit(10).unwrap() - 1).collect();

    let lets: String = i.chars().filter(|x| !x.is_ascii_digit()).into_iter().collect();
    let asciis = lets.as_bytes();

    if nums.len() != 2 || asciis.len() != 2 {
        panic!("not 2 of each letter and nu,ber, {:?}, {:?}", nums, asciis);
    }

    let mut out: Vec<u8> = vec![];

    for x in 0..=1 {
        out.push(nums[x] as u8 * 8 + (asciis[x] - 97));
    }

    return Move { start: out[0], end: out[1] };
}

fn display_board(board: &Board, thing: u8) {
    if DISPLAY_GRID {
        let vbar = "\u{2502}".to_owned();
        let hbar = "\u{2500}".to_owned();
        let ul = "\u{250C}".to_owned();
        let ur = "\u{2510}".to_owned();
        let ll = "\u{2514}".to_owned();
        let lr = "\u{2518}".to_owned();
        let nt = "\u{252C}".to_owned();
        let st = "\u{2534}".to_owned();
        let wt = "\u{251C}".to_owned();
        let et = "\u{2524}".to_owned();
        let plus = "\u{253C}".to_owned();

        let h3 = hbar.repeat(3);

        let t1 = h3.clone() + &nt;
        let t2 = h3.clone() + &plus;
        let t3 = h3.clone() + &st;

        let topline = ul.clone() + &t1.repeat(7) + &h3 + &ur;
        let midline = wt.clone() + &t2.repeat(7) + &h3 + &et;
        let botline = ll.clone() + &t3.repeat(7) + &h3 + &lr;

        let mut out: [ String; 17 ] = Default::default();

        out[0] = topline;
        out[16] = botline;

        let empty_piece = Piece { id: NONE, index: 0 };
        let mut temp = "\u{2502}".to_owned();
        let mut index = 15;

        for i in 0..64 {
            if index % 2 == 0 {
                out[index] = midline.clone();
                index -= 1;
            }

            temp.push_str(&format!(" {} ", 
                if board.squares[i].has_trait(thing) {
                    board.squares[i].to_pretty_char().to_string()
                } else {
                    empty_piece.to_pretty_char().to_string()
                }
            ));

            temp.push_str("\u{2502}");
            if i % 8 == 7 {
                out[index] = temp.clone();
                index -= 1; temp = "\u{2502}".to_owned();
                //if i <= 56 { index -= 1; temp = "\u{2502}".to_owned(); };
            }
        }


        /*
        for mut element in board.squares {
            if !element.has_trait(thing) {
                element = empty_piece;
            }
            sep.push_str(&format!(" {} ", &element.to_pretty_char().to_string()));
            sep.push_str("\u{2502}");
            index += 1;

            if index % 8 == 0 {
                if index <= 56 {
                    sep.push_str(&format!("\n{}\n\u{2502}", midline));
                } else {
                    sep.push_str("\n");
                }
                
            }
        }
        sep.push_str(&format!("{}", topline)); */

        for each in out {
            println!("{}", each);
        }
        
    } else {

        let mut sep = format!("");
        let mut index = 0;

        let empty_piece = Piece { id: NONE, index: 0 };

        for mut element in board.squares {
            if !element.has_trait(thing) {
                element = empty_piece;
            }
            sep.push_str(&format!("{} ", &element.to_pretty_char().to_string()));
            index += 1;

            if index % 8 == 0 {
                sep.push_str("\n");
            }
        }
        println!("{}", sep)
    }
}

struct Move {
    start: u8,
    end:   u8,
}

impl Move {
    fn display(&self) -> String {
        return format!(
            "{}{}{}{}",
            (&self.start % 8 + 97) as char,
            &self.start/8 + 1,
            (&self.end % 8 + 97) as char,
            &self.end/8 + 1
        )
    }
}

fn piece_from_char(chr: char) -> u8 {
    return match chr.to_ascii_lowercase() {
        'k' => KING,
        'p' => PAWN,
        'n' => KNIGHT,
        'b' => BISHOP,
        'r' => ROOK,
        'q' => QUEEN,
        _ => panic!("ur mum")

    };
}

fn load_pos() -> Board {
    let mut board: [ Piece; 64 ] = [ Piece { id: 0, index: 0 }; 64 ];

    for i in 0..64 {
        board[i] = Piece { id: 0, index: i as u8 };
    }

    let array: [&str; 6] = SETUP.split(" ").collect::<Vec<&str>>().try_into().unwrap();

    let mut file: u32 = 0;
    let mut rank: u32 = 7;

    for symbol in array[0].chars() {
        if symbol == '/' {
            file = 0;
            rank -= 1;
        } else if symbol.is_numeric() {
            file += symbol.to_digit(10).unwrap();
        } else {
            let piece_color: u8 = if symbol.is_uppercase() { WHITE } else { BLACK };
            let piece_type: u8 = piece_from_char(symbol);
            board[(rank * 8 + file) as usize] = Piece { id: piece_color | piece_type, index: (rank * 8 + file) as u8 };
            file += 1;
        }
    }

    let start_index = if array[1] == "w" { 0 } else { 1 };



    return Board { squares: board, turn_index: start_index, enpassantables: [[ true; 8 ]; 2] };
}

fn precompute_move_data() -> [[u8; 8]; 64] {
    let mut num_squares_to_edge: [[u8; 8]; 64] = [[0; 8]; 64];

    for file in 0..8 {
        for rank in 0..8 {
            let north: u8 = 7 - rank;
            let east: u8 = 7 - file;
            let south: u8 = rank;
            let west: u8 = file;

            num_squares_to_edge[(rank * 8 + file) as usize] = [
                north,
                east,
                south,
                west,
                north.min(east),
                south.min(east),
                south.min(west),
                north.min(west),
            ]
        }
    }

    return num_squares_to_edge;
}

fn main() {

    /*  CHECKLIST
            - Add enpassant to move generator
            - same for castling
            - let both sides be playable

    */

    println!();
    
    let mut board = load_pos();
    let precomputed_move_data: [[u8; 8]; 64] = precompute_move_data();
    board.display();

    println!("{} to move:", if board.color_to_move() == WHITE { "White" } else { "Black" });
    //let mut boards: Vec<Board> = vec![board];

    //board.display_all_moves();

    let mut boards = vec![board];
    for i in 0..1 {
        let now = Instant::now();
        
        

        let mut total = 0;
        for j in 0..boards.len() {
            let m = boards[j].gen_moves();
            total += m.len();

            for k in m {
                println!("{}", board.check_legality(&k))
            }
        }

        println!("{}, in {} ns, {}", total, now.elapsed().as_millis(), i);

        

        boards = next_layer(&boards);
    }

    let m = board.gen_moves();
    println!("{}", m[m.len() - 1].display());
    println!("{}", board.check_legality(&m[m.len() - 1]));

    

    for i in boards {
        //i.display();
    }
    
    

    /*loop {
        let mut input = String::new();
        print!("\x1B[2J\x1B[1;1H");
        io::stdin().read_line(&mut input).unwrap();
        board.make(&parse_move(&input));
        board.display();

        let moves = board.gen_moves();
        let movee = moves.choose(&mut rand::thread_rng()).unwrap();
        board.make(movee);

        println!("{}", movee.display());

        board.display();
    }*/

    

    













    /*
    for i in &boards {
        i.display();
    }

    boards = vec![ board; moves.len() ];

    for i in 0..moves.len() {
        boards[i].make(&moves[i]);
    }
    */

    
    /*use std::{thread, time};

    let ten_millis = time::Duration::from_millis(200);
    
    let mut i = 0;
    loop {
        print!("\x1B[2J\x1B[1;1H");
        boards[i % boards.len()].display();
        
        i += 1;
        thread::sleep(ten_millis);

        if i > 10000 {
            break;
        }
    }*/
    

    //for i in 'a'..= 'h' {
    //    println!("{}", i)
    //}
    
}