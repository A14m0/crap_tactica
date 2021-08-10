// defines our units and stuff

mod team;
mod attack;
mod position;
mod errors;
pub use team::Team;
pub use attack::Attack;
pub use position::Position;
pub use errors::ErrorOut;

/// defines a unit 
#[derive(Clone)]
pub struct Unit{
    name:           String,
    entity_id:      u64,
    team:           Team,
    health:         u64,
    attacks:        Vec<Attack>,
    position:       Position,
    action_count:   u64
}

impl std::fmt::Display for Unit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[{}] {} - {} hp", self.team, self.name, self.health)
    }
}


/// enum for returning if a unit died while dealing damage
pub enum DamageStatus {
    Alive,
    Dead
}

// begin implementing stuff for our units
impl Unit {
    /// creates a new custom unit 
    pub fn new(
        name:       String,
        entity_id:  u64,
        team:       Team,
        health:     u64,
        attacks:    Vec<Attack>,
        position:   Position,
    ) -> Self {
        let action_count = 0u64;
        Unit {
            name,
            entity_id,
            team,
            health,
            attacks,
            position,
            action_count
        }
    }

    /// creates a new unit with default properties 
    pub fn new_default(
        name:       String,
        entity_id:  u64,
        team:       Team,
        position:   Position
    ) -> Self {
        let health = 100;
        let action_count = 0u64;
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
            entity_id,
            team,
            health,
            attacks,
            position,
            action_count
        }
    }

    /// returns the position of the unit
    pub fn position(&self) -> Position {
        self.position
    }

    /// returns the entity_id of the unit 
    pub fn entity_id(&self) -> u64 {
        self.entity_id
    }

    /// returns the team of the unit 
    pub fn team(&self) -> Team {
        self.team
    }

    /// returns the action count of the unit 
    pub fn action_count(&self) -> u64 {
        self.action_count
    }
    
    /// updates the action count of the unit 
    pub fn inc_action_count(&mut self) {
        self.action_count += 1;
    }

    /// returns the name of the unit 
    pub fn name(&self) -> String {
        self.name.clone()
    }

    /// returns the name of the unit 
    pub fn attacks(&self) -> Vec<Attack> {
        self.attacks.clone()
    }

    /// returns the hp of the unit 
    pub fn health(&self) -> u64 {
        self.health
    }

    /// deals damage to the unit, returning if its still alive or not
    pub fn deal_damage(&mut self, damage: u64) -> DamageStatus {
        // see if the damage would kill us
        if damage >= self.health {
            self.health = 0;
            return DamageStatus::Dead;
        }

        self.health -= damage;
        DamageStatus::Alive
    }

}