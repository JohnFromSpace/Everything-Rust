use rand::Rng;
use std::io;

const DAYS_IN_A_WEEK: usize = 7;
const WEEKS_IN_A_YEAR: usize = 52;

#[derive(Debug)]
struct Player {
    name: String,
    proficiency: f64,
    cumulative_score: usize,
}

impl Player {
    fn new(name: &str, proficiency: f64) -> Self {
        Player {
            name: name.to_string(),
            proficiency,
            cumulative_score: 0,
        }
    }

    fn throw_dart(&self) -> usize {
         // Calculate the score based on proficiency with some randomization
        let base_score = (self.proficiency * 20.0) as usize;
        let randomization = rand::thread_rng().gen_range(-5.0..5.0);
        let final_score = base_score as isize + randomization as isize;

        if final_score < 0 {
            0
        } else if final_score > 60 {
            60
        } else {
            final_score as usize
        }    
    }
}

fn main() {
    // Set up player and AI
    let player_name = get_input("Enter your name: ");
    let player_proficiency = get_input_f64("Enter your proficiency level (0.0 to 1.0): ");
    let player = Player::new(&player_name, player_proficiency);

}
