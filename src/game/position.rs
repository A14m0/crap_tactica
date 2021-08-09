/// defines a position on the board
#[derive(Copy,Clone,PartialEq)]
pub struct Position {
    x: usize,
    y: usize
} 

impl Position {
    /// creates a new position
    pub fn new(x:usize, y:usize) -> Self {
        Position{x,y}
    }

    /// returns the x coordinate
    pub fn x(&self) -> usize {
        self.x
    }

    /// returns the y coordinate
    pub fn y(&self) -> usize {
        self.y
    }

    /// updates the position
    pub fn update(&mut self, x: usize, y: usize) {
        self.x = x;
        self.y = y;
    }

    /// calculates the distance between two positions
    pub fn distance(&self, p: Self) -> f64 {
        let x_max: usize;
        let x_min: usize;
        let y_max: usize;
        let y_min: usize;

        if self.x() > p.x() {
            x_max = self.x();
            x_min = p.x();
        } else {
            x_max = p.x();
            x_min = self.x();
        }

        if self.y() > p.y() {
            y_max = self.y();
            y_min = p.y();
        } else {
            y_max = p.y();
            y_min = self.y();
        }
        
        // return the distance 
        (
            ((x_max-x_min) as f64).powf(2f64) + // a^2 + ...
            ((y_max-y_min) as f64).powf(2f64)   // b^2...
        ).sqrt()                                // = c^2, so c=c.sqrt()
    }
}