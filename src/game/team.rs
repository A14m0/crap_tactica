// Defines our team definitions and whatnot

// define the teams enum
#[derive(PartialEq, Copy, Clone)]
pub enum Team {
    Redfor,
    Bluefor
}


/// display format implementation
impl std::fmt::Display for Team {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let printable = match self { 
            Team::Redfor => "Redfor",
            Team::Bluefor => "Bluefor"
        };

        write!(f, "{}", printable)
    }
}