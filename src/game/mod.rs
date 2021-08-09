// defines our units and stuff

pub mod team;
pub mod attack;
pub mod position;
use team::Team;
use attack::Attack;
use position::Position;

/// defines a unit 
pub struct Unit{
    name:       String,
    entity_id:  u64,
    team:       Team,
    health:     u64,
    attacks:    Vec<Attack>,
    position:   Position,
}


/// enum for returning if a unit died while dealing damage
enum DamageStatus {
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
        Unit {
            name,
            entity_id,
            team,
            health,
            attacks,
            position
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
            position
        }
    }

    /// returns the position of the unit
    pub fn position(&self) -> Position {
        self.position
    }

    /// returns the entity_id of the unit 
    pub fn entity_id(&self) -> u64 {
        self.entity_id()
    }

    /// deals damage to the unit, returning if its still alive or not
    fn deal_damage(&mut self, damage: u64) -> DamageStatus {
        // see if the damage would kill us
        if damage > self.health {
            self.health = 0;
            return DamageStatus::Dead;
        }

        self.health -= damage;
        DamageStatus::Alive
    }

    /// make unit do attack
    pub fn do_attack(
        &self, 
        attack: Attack, 
        target: &mut Self
    ) -> Result<(),String>{
        // make sure the target is within range
        let distance = self.position()
                           .distance(target.position());

        // make sure we are within range for the attack
        if distance > attack.range() {
            return Err("Target out of range".to_string());
        }

        // do the attack (i.e. deal damage to target)
        match target.deal_damage(attack.damage()) {
            DamageStatus::Alive => (),
            DamageStatus::Dead => () // todo: do something when a target is dead
        };

        Ok(())
    }
}