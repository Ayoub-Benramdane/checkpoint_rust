#[derive(Debug)]
pub struct ChessPosition {
    pub rank: i32,
    pub file: i32,
}

#[derive(Debug)]
pub struct Queen {
    pub position: ChessPosition,
}

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        Some(ChessPosition { rank, file })
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Queen { position }
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        if
            self.position.rank == other.position.rank ||
            self.position.file == other.position.file ||
            (self.position.rank - other.position.rank).abs() ==
                (self.position.file - other.position.file).abs()
        {
            true
        } else {
            false
        }
    }
}
