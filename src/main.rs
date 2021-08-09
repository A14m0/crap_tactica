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
    cmd: String,
    action: fn (game: &mut Game, unit_id: u64) -> game::ErrorOut,
}

//////////////// ACTION CMDS //////////////////////////
/// Attack a target
fn attack(game: &mut Game, unit_id: u64) -> game::ErrorOut {
    println!("Attacking!");
    game::ErrorOut::SUCCESS
}

/// Move unit to new position
fn move_unit(game: &mut Game, unit_id: u64) -> game::ErrorOut {
    println!("Moving!");
    game::ErrorOut::SUCCESS
}


/// defines our game grid
struct Game {
    size:       u64,
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
                    "Billy but Bad".to_string(), 
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
                for soldier in &units{
                    if soldier.team() == game::Team::Redfor {
                        grid[row as usize].push(soldier.entity_id());
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
                for soldier in &units{
                    if soldier.team() == game::Team::Bluefor {
                        grid[row as usize].push(soldier.entity_id());
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
                action: attack
            }
        );
        // "move"
        commands.push(
            Command {
                cmd: "move".to_string(),
                action: move_unit
            }
        );


        Game {
            size,
            grid,
            units,
            commands
        }
    }

    /// returns the number of turns each player should get before looing again
    fn count_player_turns(&self) -> usize {
        self.units.len() / 2
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
    let p2: game::Team;
    // see who gets the first turn
    if rand::random() {
        println!("[B] BLUEFOR has seized the initiative!");
        p1 = game::Team::Bluefor;
        p2 = game::Team::Redfor;
    } else {
        println!("[R] REDFOR has seized the initiative!");
        p1 = game::Team::Redfor;
        p2 = game::Team::Bluefor;
    }

    // begin main game loop
    loop {
        println!("[+] Top of the initiative order");
        // loop over each pair of player turns
        for _pair in 0..player_turns{
            // find the player's units
            let s1 = match g.find_next_unit(p1) {
                Ok(a) => a, 
                Err(e) => panic!("Failed to get next unit: {}", e)
            };
            let s2 = match g.find_next_unit(p2) {
                Ok(a) => a, 
                Err(e) => panic!("Failed to get next unit: {}", e)
            };

            // get the first player's input 
            loop {
                // get the string
                print!("[{}] {} > ", s1.team(), s1.name());
                std::io::stdout().flush().unwrap();
                let mut ustr = String::new();
                std::io::stdin().read_line(&mut ustr).unwrap();
                let ustr = ustr.replace("\n", "");

                // look for the command
                let mut rcode: game::ErrorOut = game::ErrorOut::NOT_FOUND;
                for comm in &game_commands {
                    if ustr == comm.cmd {
                        rcode = (comm.action)(&mut g, s1.entity_id());
                    }
                } 

                // deterine command outcome
                match rcode {
                    game::ErrorOut::SUCCESS => {
                        g.incr_unit_action(s1.entity_id());
                        break
                    },
                    game::ErrorOut::NOT_FOUND => println!("[-] Command not found"),
                    _ => println!("[-] Unexpected error...")
                }
            }

            // get the second player's input 
            loop {
                print!("[{}] {} > ", s2.team(), s2.name());
                std::io::stdout().flush().unwrap();
                let mut ustr = String::new();
                std::io::stdin().read_line(&mut ustr).unwrap();
                let ustr = ustr.replace("\n", "");

                let mut rcode: game::ErrorOut = game::ErrorOut::NOT_FOUND;
                for comm in &game_commands {
                    if ustr == comm.cmd {
                        rcode = (comm.action)(&mut g, s1.entity_id());
                    }
                } 

                match rcode {
                    game::ErrorOut::SUCCESS => break,
                    game::ErrorOut::NOT_FOUND => println!("[-] Command not found"),
                    _ => println!("[-] Unexpected error...")
                }
            }

        }

        // get the user's input
    }

}
