// include our units stuff
mod game;

/// definition of terrain entity_id
const TERRAIN_ID: u64 = 0xffffffffffffffff;

/// defines a grid square for our game
struct GridSquare {
    x: usize,
    y: usize,
    entity_id: u64
}

/// defines our game grid
struct Game {
    grid: Vec<Vec<GridSquare>>,
    units: Vec<game::Unit>,
    terrain: Vec<GridSquare>
}

impl Game {
    /// creates a new default game
    fn new_default(size: u64) {
        
    }
}

fn main() {
}
