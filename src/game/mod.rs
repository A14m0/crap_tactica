// defines our units and stuff

pub mod team;
pub mod attack;
pub mod position;
use team::Team;
use attack::Attack;
use position::Position;

/// defines a unit 
pub struct Unit{
    name: String,
    team: Team,
    health: u64,
    attacks: Vec<Attack>,
    position: Position
}

// begin implementing stuff for our units
impl Unit {
    /// creates a new custom unit 
    pub fn new(
        name: String,
        team: Team,
        health: u64,
        attacks: Vec<Attack>,
        position: Position
    ) -> Self {
        Unit {
            name,
            team,
            health,
            attacks,
            position
        }
    }

    /// creates a new unit with default properties 
    pub fn new_default(
        name: String,
        team: Team,
        position: Position
    ) -> Self {
        let health = 100;
        let mut attacks: Vec<Attack> = Vec::new();
        attacks.push(Attack::new(
            "Fight".to_string(),
            50,
            1f64
        ));
        attacks.push(Attack::new(
            "Shoot".to_string(),
            30,
            10f64
        ));

        Unit {
            name,
            team,
            health,
            attacks,
            position
        }
    }

    /// returns the position of the unit
    pub fn get_pos(&self) -> Position {
        self.position
    }

    /// make unit do attack
    pub fn do_attack(&self, attack: Attack, target: &mut Self) {
        // make sure the target is within range
        let distance = self.get_pos()
                           .distance(target.get_pos());

        if distance > attack.range() {
        
        }
    }
}