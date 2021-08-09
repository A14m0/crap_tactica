/// defines an attack a unit can do
pub struct Attack {
    name: String,
    damage: usize,
    range: f64,
    // TODO: Can add effects creators down here at some point
}

// begin implementing our Attack functions
impl Attack {
    /// creates a new attack 
    pub fn new(
        name: String,
        damage: usize,
        range: f64
    ) -> Self {
        Attack {
            name, 
            damage,
            range
        }
    }

    /// returns the range of the attack
    pub fn range(&self) -> f64 {
        self.range
    }
}