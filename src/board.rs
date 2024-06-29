use crate::*;

#[derive(Debug, Clone, Copy)]
pub struct Board {
    pub squares: [ Piece; 64 ],
    pub turn_index: u32,
    pub enpassantables: [[ bool; 8 ]; 2],
}

impl Board {
    pub fn color_to_move(&self) -> u8 {
        return if self.turn_index % 2 == 0 { WHITE } else { BLACK };
    }

    pub fn piece_grid(&self, cond: u8)/* -> [ Piece; 64 ] */ {
        let new_board: [ Piece; 64 ] = [ Piece { id: 0, index: 0 }; 64];
    }

    pub fn display(&self) {
        display_board(&self, 255);
    }

    pub fn display_with_thing(&self, thing: u8) {
        display_board(&self, thing);
    }

    pub fn make(&mut self, movee: &Move) {
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

    pub fn unmake(&mut self, movee: &Move) {
        self.squares.swap(movee.start as usize, movee.end as usize);
        self.turn_index -= 1;
    }

    pub fn gen_sliding_moves(&self, piece: &Piece) -> Vec<Move> {
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

    pub fn gen_other_moves(&self, piece: &Piece) -> Vec<Move> {
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
                    _ => panic!("shouldnt be {}", piece.to_pretty_char()),
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
    pub fn gen_moves(&self) -> Vec<Move> {
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

    pub fn gen_moves_at(&self, index: u8) -> Vec<Move> {
        let mut moves: Vec<Move> = vec![];
        let piece = self.squares[index as usize];

        if piece.is_sliding_piece() {
            moves.append(&mut self.gen_sliding_moves(&piece));
        } else {
            moves.append(&mut self.gen_other_moves(&piece));
        }

        return moves;
    }

    pub fn check_legality(&self, movee: &Move) -> bool {
        let mut temp = self.clone();

        
        if !(temp.gen_moves().contains(movee)) {
            return false;
        }

        temp.make(movee);
        let moves_to_check = temp.gen_moves();

        for i in moves_to_check {
            if temp.squares[i.end as usize].piece_type() == KING {
                return false;
            }
        }

        return true;
    }

    pub fn display_all_moves(&self) {
        let moves = self.gen_moves();
        let mut boards = vec![ self.to_owned(); moves.len() ];

        for i in 0..moves.len() {
            boards[i].make(&moves[i]);
            boards[i].display();
        }
    }

}