use rand::prelude::*;
use rand::Rng;
use std::io::Write;
// include our units stuff
mod game;

/// definition of terrain and empty entity_id
const TERRAIN_ID:   u64 = 0xffffffffffffffff;
const EMPTY_ID:     u64 = 0x7fffffffffffffff;


/// defines a command usable in the game
#[derive(Clone)]
struct Command {
    cmd:    String,
    help:   String,
    action: fn (game: &mut Game, unit_id: u64) -> game::ErrorOut,
}

/// function that helps simplify the fetching of user input
fn input(print: String) -> String {
    // get the string
    print!("{}", print);
    std::io::stdout().flush().unwrap();
    let mut ustr = String::new();
    std::io::stdin().read_line(&mut ustr).unwrap();
    let ustr = ustr.replace("\n", "");

    ustr
}

//////////////// ACTION CMDS //////////////////////////
/// Attack a target
fn attack(game: &mut Game, unit_id: u64) -> game::ErrorOut {
    // get the attacker's unit information
    let s = match game.get_unit(unit_id) {
        Ok(a) => a,
        Err(e) => {
            println!("Failed to get unit: {}", e);
            return game::ErrorOut::FAILED_GENERIC
        }
    };
    
    loop {
        // determine which attack we should use
        println!("{} can use the following attacks:", s.name());
        let mut idx = 0;
        for attack in s.attacks() {
            idx += 1;
            println!(
                "\t{}. {} (range {}, dmg {})", 
                idx, 
                attack.name(),
                attack.range(),
                attack.damage()
            );
        }

        let ustr = input(format!("Attack with which number > "));

        // get the index of the attack to use
        let attack_idx = match ustr.parse::<usize>() {
            Ok(a) => a-1,
            Err(_) => {
                println!("[-] That was not a valid number. Select the attack by the number to the left of it");
                continue
            }
        };

        // now try to figure out what targets are within range and add them to a vector
        let mut uctr = 0;
        let mut tgt_vec: Vec<game::Unit> = Vec::new();
        for u in game.units() {
            // check if the attack is within range
            if s.attacks()[attack_idx as usize].range() >= s.position().distance(u.position()) 
                && u.team() != s.team() {
                uctr += 1;
                println!("\t{}: {}", uctr, u);
                tgt_vec.push(u);
            }
        }

        let ustr = input(format!("Attack which unit > "));

        let target_idx = match ustr.parse::<usize>() {
            Ok(a) => a-1,
            Err(_) => {
                println!("[-] That was not a valid number. Select the attack by the number to the left of it");
                continue
            }
        };
        let target_id = tgt_vec[target_idx].entity_id();
        // try to do the attack
        match game.do_attack(
            s.entity_id(),
            target_id,
            s.attacks()[attack_idx as usize].clone()
        ) {
            Ok(a) => match a {
                game::DamageStatus::Alive => {
                    let target_unit = game.get_unit(target_id).unwrap();
                    println!("[+] Attack hit! {} is now at {} hp!", 
                    target_unit.name(), target_unit.health());
                    return game::ErrorOut::SUCCESS;
                },
                game::DamageStatus::Dead => {
                    println!("[+] Enemy was killed!");
                    return game::ErrorOut::SUCCESS;
                }
            },
            Err(e) => println!("Failed to do attack: {}", e)

        }
    }
}

/// Move unit to new position
fn move_unit(game: &mut Game, unit_id: u64) -> game::ErrorOut {
    loop {
        // print movement options
        println!("Movement options: ");
        println!("\t1. Up");
        println!("\t2. Down");
        println!("\t3. Left");
        println!("\t4. Right");
    
        // get the user's selection and convert it to an enum
        let ustr = input(format!("Pick a movement > "));
        let movement_val = match ustr.parse::<usize>() {
            Ok(a) => a-1,
            Err(_) => {
                println!("[-] That was not a valid number. Select the attack by the number to the left of it");
                continue
            }
        };

        let mov = match movement_val {
            0 => game::Movement::Up,
            1 => game::Movement::Down,
            2 => game::Movement::Left,
            3 => game::Movement::Right,
            _ => {
                println!("[-] That was not a valid movement selection.");
                continue
            }
        };

        // try to move the unit
        match game.move_unit(unit_id, mov) {
            game::ErrorOut::SUCCESS => break,
            _ => println!("[-] Cannot move there!")
        }
    }

    game.incr_unit_action(unit_id);
    game::ErrorOut::SUCCESS
}

/// Move unit to new position
fn help(game: &mut Game, _unit_id: u64) -> game::ErrorOut {
    println!("Available Commands:");
    for comm in game.commands() {
        println!("\t{}: \t{}", comm.cmd, comm.help);
    }
    game::ErrorOut::SUCCESS_INCOMPLETE
}

/// Prints the health of the unit
fn health(game: &mut Game, unit_id: u64) -> game::ErrorOut {
    let s = game.get_unit(unit_id).unwrap();
    println!("{} is at {} hitpoints", s.name(), s.health());
    game::ErrorOut::SUCCESS_INCOMPLETE
}

/// Prints the position of the unit
fn position(game: &mut Game, unit_id: u64) -> game::ErrorOut {
    let s = game.get_unit(unit_id).unwrap();
    println!("{} is at position {}", s.name(), s.position());
    game::ErrorOut::SUCCESS_INCOMPLETE
}

/// Move unit to new position
fn end(_game: &mut Game, _unit_id: u64) -> game::ErrorOut {
    println!("[-] Turn ended!");
    game::ErrorOut::SUCCESS
}


/// defines our game grid
struct Game {
    grid:       Vec<Vec<u64>>,
    units:      Vec<game::Unit>,
    commands:   Vec<Command>
}

impl Game {
    /// creates a new default game
    fn new_default() -> Self {
        let size = 32u64;
        let mut grid: Vec<Vec<u64>> = Vec::with_capacity(size as usize);
        let mut units: Vec<game::Unit> = Vec::new();
        let mut commands: Vec<Command> = Vec::new();

        // create our BLUEFOR units
        for i in 0..3 {
            units.push(
                game::Unit::new_default(
                    format!("Billy #{}", i+1), 
                    i, 
                    game::Team::Bluefor,
                    // note we will update these later on when we generate the grid 
                    game::Position::new(0,0)
                )
            );
        }

        // create our REDFOR units
        for i in 0..3 {
            units.push(
                game::Unit::new_default(
                    format!("Billy but Bad #{}", i+1), 
                    i+3, 
                    game::Team::Redfor,
                    // note we will update these later on when we generate the grid 
                    game::Position::new(0,0)
                )
            );
        }

        // generate the grid and populate it
        // for each row in 0 .. size
        for row in 0..size {
            grid.push(Vec::with_capacity(size as usize));

            // check if the row is the deployment zone for REDFOR
            if row == 0 {
                // add all REDFOR soldiers to the corner and over
                let mut rctr = 0;
                for soldier in units.iter_mut(){
                    if soldier.team() == game::Team::Redfor {
                        grid[row as usize].push(soldier.entity_id());
                        soldier.move_unit(
                            game::Position::new (
                                row as usize , 
                                rctr
                            )
                        );
                        rctr += 1;
                    }
                }
                for _ in 0..size as usize - grid[row as usize].len() {
                    grid[row as usize].push(EMPTY_ID);
                }

            } 
            // check if the row is the deployment zone for BLUEFOR
            else if row == size-1 {
                let mut scount = 0;
                for soldier in &units {
                    if soldier.team() == game::Team::Bluefor{
                        scount += 1;
                    }
                }

                // fill the first part of the row with empty tiles
                for _ in 0..size as usize - scount {
                    grid[row as usize].push(EMPTY_ID);
                }

                // add all BLUEFOR soldiers to the corner and over
                let mut bctr = size as usize - scount;
                for soldier in units.iter_mut(){
                    if soldier.team() == game::Team::Bluefor {
                        grid[row as usize].push(soldier.entity_id());
                        soldier.move_unit(
                            game::Position::new (
                                row as usize , 
                                bctr,
                            )
                        );
                        bctr += 1;
                    }
                }
            } else {
                // loop over the length it should be
                for _ in 0..size {
                    // randomly generate an integer
                    let val = thread_rng().gen_range(0..10);
                    
                    // if the value is part of our target, add a terrain
                    if val % 5 == 0 {
                        grid[row as usize].push(TERRAIN_ID);
                    } else {
                        grid[row as usize].push(EMPTY_ID);
                    }
                }
            }
        }

        // create all of our commands
        // "attack"
        commands.push(
            Command{
                cmd: "attack".to_string(),
                help: "Attack an enemy".to_string(),
                action: attack
            }
        );
        // "move"
        commands.push(
            Command {
                cmd: "move".to_string(),
                help: "Move to a new position".to_string(),
                action: move_unit
            }
        );
        // "health"
        commands.push(
            Command {
                cmd: "health".to_string(),
                help: "Shows the health of your unit".to_string(),
                action: health
            }
        );
        // "position"
        commands.push(
            Command {
                cmd: "position".to_string(),
                help: "Shows the position of your unit".to_string(),
                action: position
            }
        );
        // "end"
        commands.push(
            Command {
                cmd: "end".to_string(),
                help: "End your unit's turn".to_string(),
                action: end
            }
        );
        // "help"
        commands.push(
            Command {
                cmd: "help".to_string(),
                help: "Shows all of the commands".to_string(),
                action: help
            }
        );


        Game {
            grid,
            units,
            commands
        }
    }

    /// returns the units currently in the game
    fn units(&self) -> Vec<game::Unit> {
        self.units.clone()
    }

    /// returns the number of turns each player should get before looping again
    fn count_player_turns(&self) -> usize {
        self.units.len()
    }

    /// returns all the available commands for the game
    fn commands(&self) -> Vec<Command> {
        self.commands.clone()
    }
    
    /// increments the action of a unit
    fn incr_unit_action(&mut self, id: u64) {
        for unit in self.units.iter_mut() {
            if unit.entity_id() == id {
                unit.inc_action_count();
            }
        }
    }

    /// returns the next mutable unit for the specified team
    fn find_next_unit(&mut self, team: game::Team) -> Result<game::Unit, String> {
        let mut lowest_init = 0xffff;
        let mut lowest_id = 0u64;
        
        // search for the soldier with the lowest init and matching team
        for u in &self.units {
            if u.action_count() < lowest_init && u.team() == team {
                lowest_init = u.action_count();
                lowest_id = u.entity_id();
            }
        }

        // loop agan and just return the unit ref
        for u in self.units.iter_mut() {
            if u.entity_id() == lowest_id {
                return Ok(u.clone());
            }
        }

        
        Err("Shouldnt be reachable...".to_string())
    }

    /// returns the unit with `id`
    fn get_unit(&self, id: u64) -> Result<game::Unit, String>{
        for unit in self.units.iter() {
            if unit.entity_id() == id {
                return Ok(unit.clone());
            }
        }

        Err("No unit with that ID found".to_string())
    }
    
    /// does the attack on behalf of the unit
    fn do_attack(
        &mut self, 
        attacker:   u64,
        target:     u64,
        attack:     game::Attack 
    ) -> Result<game::DamageStatus, String>{
        // find the attacker and target
        let mut attacker_idx = 0;
        for unit in self.units.iter() {
            if unit.entity_id() == attacker {
                break;
            }
            attacker_idx += 1;
        }
        // remove mutability
        let attacker_idx = attacker_idx;

        let mut target_idx = 0;
        for unit in self.units.iter() {
            if unit.entity_id() == target {
                break;
            }
            target_idx += 1;
        }
        // remove mutability
        let target_idx = target_idx;

        
        // make sure the target is within range
        let distance = self.units[attacker_idx].position()
                           .distance(self.units[target_idx].position());

        // make sure we are within range for the attack
        if distance > attack.range() {
            return Err("Target out of range".to_string());
        }


        // increment the unit action
        self.incr_unit_action(attacker);
        
        // now try to do the attack
        match self.units[target_idx]
            .deal_damage(attack.damage()) {
            game::DamageStatus::Alive => {
                Ok(game::DamageStatus::Alive)
            },
            game::DamageStatus::Dead => {
                // remove the target unit from the list
                self.units.remove(target_idx);
                Ok(game::DamageStatus::Dead)
            }
        }
        
    }

    /// prints the grid to the screen
    fn print_grid(&self) {
        // loop over each row
        for row in 0..self.grid.len() {
            let row = self.grid.len() - row -1;
            print!("|");
            // loop over each cell
            for cell in 0..self.grid[row].len() {
                let id = self.grid[row][cell];
                if id == EMPTY_ID {
                    print!(" |");
                } else if id == TERRAIN_ID {
                    print!("^|");
                } else {
                    let unit = self.get_unit(id).unwrap();
                    match unit.team() {
                        game::Team::Redfor => print!("R|"),
                        game::Team::Bluefor => print!("B|")
                    }
                }
            }
            println!("");
        }
    }

    /// attempts to move a unit 
    fn move_unit(&mut self, unit_id: u64, mov: game::Movement) -> game::ErrorOut {
        let mut unit = self.get_unit(unit_id).unwrap();

        println!("Current pos {}", unit.position());
        match mov {
            game::Movement::Up => {
                if unit.position().x() == self.grid.len() {
                    return game::ErrorOut::FAILED_GENERIC;
                }
                let above = unit.position().x() + 1;
                // note this catches both terrain and friendly units in the way
                if self.grid[above][unit.position().y()] != EMPTY_ID {
                    return game::ErrorOut::FAILED_GENERIC;
                }

                // position is valid, update internal stuff
                self.grid[above][unit.position().y()] = unit_id;
                self.grid[above-1][unit.position().y()] = EMPTY_ID;

                println!("Current pos {}x{}", above, unit.position().y());

                unit.move_unit(
                    game::Position::new(
                        above,
                        unit.position().y()
                    )
                );
            },
            game::Movement::Down => {
                // bounds check it
                if unit.position().x() == 0 {
                    return game::ErrorOut::FAILED_GENERIC;
                }
                let below = unit.position().x() - 1;
                // note this catches both terrain and friendly units in the way
                if self.grid[below][unit.position().y()] != EMPTY_ID {
                    return game::ErrorOut::FAILED_GENERIC;
                }

                // position is valid, update internal stuff
                self.grid[below][unit.position().y()] = unit_id;
                self.grid[below+1][unit.position().y()] = EMPTY_ID;

                println!("Current pos {}x{}", below, unit.position().y());

                unit.move_unit(
                    game::Position::new(
                        below,
                        unit.position().y()
                    )
                );
            },
            game::Movement::Left => {
                // bounds check it
                if unit.position().y() == 0 {
                    return game::ErrorOut::FAILED_GENERIC;
                }
                let aside = unit.position().y() - 1;
                // note this catches both terrain and friendly units in the way
                if self.grid[unit.position().x()][aside] != EMPTY_ID {
                    return game::ErrorOut::FAILED_GENERIC;
                }

                // position is valid, update internal stuff
                self.grid[unit.position().x()][aside] = unit_id;
                self.grid[unit.position().x()][aside+1] = EMPTY_ID;

                println!("Current pos {}x{}", unit.position().x(), aside);

                unit.move_unit(
                    game::Position::new(
                        unit.position().x(),
                        aside
                    )
                );
            },
            game::Movement::Right => {
                // bounds check it
                if unit.position().y() == self.grid.len() {
                    return game::ErrorOut::FAILED_GENERIC;
                }
                let aside = unit.position().y() + 1;
                // note this catches both terrain and friendly units in the way
                if self.grid[unit.position().x()][aside] != EMPTY_ID {
                    return game::ErrorOut::FAILED_GENERIC;
                }

                // position is valid, update internal stuff
                self.grid[unit.position().x()][aside] = unit_id;
                self.grid[unit.position().x()][aside-1] = EMPTY_ID;

                println!("Current pos {}x{}", unit.position().x(), aside);

                unit.move_unit(
                    game::Position::new(
                        unit.position().x(),
                        aside
                    )
                );
            },
        }

        game::ErrorOut::SUCCESS
    }
}








fn main() {
    // create our game struct
    let mut g = Game::new_default();
    let game_commands = g.commands();

    // begin the main player loop
    let player_turns = g.count_player_turns();
    println!(
        "[+] Each player shall get {} turns before going to top of initative",
        player_turns
    );

    let p1: game::Team;
    // see who gets the first turn
    if rand::random() {
        println!("[B] BLUEFOR has seized the initiative!");
        p1 = game::Team::Bluefor;
    } else {
        println!("[R] REDFOR has seized the initiative!");
        p1 = game::Team::Redfor;
    }

    // begin main game loop
    loop {
        println!("[+] Top of the initiative order");
        // loop over each pair of player turns
        let mut curr_team = p1;
        for _pair in 0..player_turns{
            g.print_grid();
            // make sure there are units for both teams available
            let mut bf_ctr = 0;
            let mut rf_ctr = 0;
            for unit in g.units() {
                match unit.team() {
                    game::Team::Redfor => rf_ctr += 1,
                    game::Team::Bluefor => bf_ctr += 1
                }
            }

            if bf_ctr == 0 {
                println!("[-] NO MORE BLUEFOR UNITS!");
                println!("[+] REDFOR WINS!");
                return;
            } 
            if rf_ctr == 0 {
                println!("[-] NO MORE REDFOR UNITS!");
                println!("[+] BLUEFOR WINS!");
                return;
            } 
            

            // find the player's units
            let s1 = match curr_team {
                game::Team::Bluefor => {
                    match g.find_next_unit(game::Team::Bluefor) {
                        Ok(a) => a, 
                        Err(e) => {
                            println!("[-] NO MORE BLUEFOR UNITS!");
                            println!("[+] REDFOR WINS!");
                            return;
                        }
                    }
                },
                game::Team::Redfor => {
                    match g.find_next_unit(game::Team::Redfor) {
                        Ok(a) => a, 
                        Err(e) => {
                            println!("[-] NO MORE REDFOR UNITS!");
                            println!("[+] BLUEFOR WINS!");
                            return;
                        }
                    }
                }
            };
            // get the player's input 
            loop {
                let ustr = input(format!("[{}] {} > ", s1.team(), s1.name()));

                // look for the command
                let mut rcode: game::ErrorOut = game::ErrorOut::NOT_FOUND;
                for comm in &game_commands {
                    if ustr == comm.cmd {
                        rcode = (comm.action)(&mut g, s1.entity_id());
                    }
                } 

                // deterine command outcome
                match rcode {
                    game::ErrorOut::SUCCESS => break,
                    game::ErrorOut::SUCCESS_INCOMPLETE => continue,
                    game::ErrorOut::NOT_FOUND => println!("[-] Command not found"),
                    _ => println!("[-] Unexpected error...")
                }
            }

            curr_team = curr_team.other_team();
        }

        // get the user's input
    }

}
