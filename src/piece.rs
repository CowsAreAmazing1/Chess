use crate::*;

#[derive(Clone, Copy, Debug)]
pub struct Piece {
    pub id: u8,
    pub index: u8,
}

impl Piece {
    pub fn is_color(&self, color: u8) -> bool {
        return (self.id & COLORMASK) == color;
    }

    pub fn color(&self) -> u8 {
        return self.id & COLORMASK;
    }

    pub fn piece_type(&self) -> u8{
        return self.id & TYPEMASK;
    }

    pub fn is_rook_or_queen(&self) -> bool {
        return (self.id & 0b110) == 0b110;
    }

    pub fn is_bishop_or_queen(&self) -> bool {
        return (self.id & 0b101) == 0b101;
    }

    pub fn is_sliding_piece(&self) -> bool {
        return (self.id & 0b100) != 0;
    }

    pub fn enemy_color(&self) -> u8 {
        return if self.color() == WHITE { BLACK } else { WHITE }
    }

    pub fn has_trait(&self, thing: u8) -> bool {
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
    

    pub fn to_char(&self) -> char {
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

    pub fn to_pretty_char(&self) -> ColoredString {
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