use crate::lib::die;
use clap::Args;

//  ====
//  ROLL
//  ====

/// Roll die
///
/// Roll die accepts input in the {n}d{S} format; where {n} is the number of dice thrown
/// and {S} is the number of sides each die has. (example: 3d8 - Three 8-sided die)
/// defaults to 1d20
///
/// Examples:
/// random die          -   Rolls one twenty-sided die
/// random die 1d6      -   Rolls one six-sided die
/// random die 3d12     -   Rolls three twelve-sided die
#[derive(Args)]
#[clap(verbatim_doc_comment)]
pub struct Roll {
    /// The dice to roll in {n}d{S} format
    die: Option<String>,
}

impl Roll {
    pub fn execute(self: &Self) {
        //  Read user-input or take default die
        let die = match self.die.to_owned() {
            Some(x) => x,
            None => String::from("1d20"),
        };

        //  Roll the die
        let result = die::roll(&die);

        //  Show results
        println!("Rolls: {:?} = {}", result, result.iter().sum::<u32>());
    }
}
