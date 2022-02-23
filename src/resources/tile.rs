use colored::Colorize;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Tile {
    Bomb,
    BombNeighbor(u8),
    Empty,
}

impl Tile {
    pub const fn is_bomb(&self) -> bool {
        matches!(self, Self::Bomb)
    }

    pub fn get_debug_string(&self) -> String {
        match self {
            Tile::Bomb => "*".bright_red(),
            Tile::BombNeighbor(v) => match v {
                1 => "1".cyan(),
                2 => "2".green(),
                3 => "3".yellow(),
                // TODO: can we not allocate here?
                _ => v.to_string().red(),
            },
            Tile::Empty => " ".normal(),
        }
        .to_string()
    }
}
