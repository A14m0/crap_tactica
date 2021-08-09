/// defines an attack a unit can do
#[derive(Clone)]
pub struct Attack {
    name: String,
    damage: u64,
    range: f64,
    // TODO: Can add effects creators down here at some point
}

// begin implementing our Attack functions
impl Attack {
    /// creates a new attack 
    pub fn new(
        name: String,
        damage: u64,
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

    /// returns the damage of the attack
    pub fn damage(&self) -> u64{
        self.damage
    }

}