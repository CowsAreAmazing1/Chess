#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_assignments)]

use crate::board::*;
use crate::piece::*;
use crate::r#move::*;

mod board;
mod piece;
mod r#move;


use std::iter::Iterator;
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

//const SETUP: &str = "8/8/8/PPP5/PK5r/PPP2N2/6P1/8 w - - 0 1";
//const SETUP: &str = "8/8/8/8/4n1n1/2n3n1/P1P1P1P1/8 w - - 0 1";
//const SETUP: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQkq - 0 1";
const SETUP: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w - - 0 1";


const DIRECTION_OFFSETS: [i8; 8] = [ 8, 1, -8, -1, 9, -7, -9, 7 ];


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
        let ul   = "\u{250C}".to_owned();
        let ur   = "\u{2510}".to_owned();
        let ll   = "\u{2514}".to_owned();
        let lr   = "\u{2518}".to_owned();
        let nt   = "\u{252C}".to_owned();
        let st   = "\u{2534}".to_owned();
        let wt   = "\u{251C}".to_owned();
        let et   = "\u{2524}".to_owned();
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

    /*
    let mut boards: Vec<Board> = vec![board];
    let moves: Vec<Move> = board.gen_moves().into_iter().filter(|x| board.check_legality(x)).collect();
    boards = vec![ board; moves.len() ];
    
    for i in 0..moves.len() {
        boards[i].make(&moves[i]);
    }
    
    for i in 0..boards.len() {
        boards[i].display();
        println!("{:?}", board.check_legality(&moves[i]));
    }*/
    






    //board.display_all_moves();

    /*let mut boards = vec![board];
    for i in 0..1 {
        let now = Instant::now();
        
        

        let mut total = 0;
        for j in 0..boards.len() {
            let m = boards[j].gen_moves();
            total += m.len();
        }

        println!("{}, in {} ns, {}", total, now.elapsed().as_millis(), i);

        boards = next_layer(&boards);
    }*/

    loop {
        let mut input = String::new();
        print!("\x1B[2J\x1B[1;1H");
        io::stdin().read_line(&mut input).unwrap();
        let mut attemptedmove = parse_move(&input);
        while !board.check_legality(&attemptedmove) {
            print!("\x1B[2J\x1B[1;1H");
            board.display();
            println!("That move isnt legal, try again: ");
            input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            attemptedmove = parse_move(&input);
        }
        
        board.make(&attemptedmove);
        //print!("\x1B[2J\x1B[1;1H");
        
        

        let moves = board.gen_moves();
        let movee = moves.choose(&mut rand::thread_rng()).unwrap();
        board.make(movee);

        println!("{}", movee.display());

        board.display();
    }

    

    













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