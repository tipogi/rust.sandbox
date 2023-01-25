#[derive(Debug)]
pub struct ChessPosition {
    file: i32, // y
    rank: i32 // x
}

#[derive(Debug)]
pub struct Queen {
    position: ChessPosition
}

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        if rank < 0 || rank > 7 { return None; }
        if file < 0 || file > 7 { return None; }
        Some(
            Self {
                rank,
                file
            }
        )
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Self { position }
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        if self.rank() == other.rank() || self.file() == other.file() {
            return true;
        } else if self.file() > other.file() {
            let diff = self.file() - other.file();
            return self.same_diff(other.rank(), diff);
        } else {
            let diff = other.file() - self.file();
            return self.same_diff(other.rank(), diff);
        }
    }

    fn same_diff(&self, rank_b: i32, diff: i32) -> bool {
        return self.rank() == rank_b + diff || rank_b == self.rank() + diff
    }

    fn file(&self) -> i32{
        self.position.file
    }

    fn rank(&self) -> i32{
        self.position.rank
    }
}


fn main() {}