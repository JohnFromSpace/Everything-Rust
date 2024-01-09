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

    let mut ai_proficiency = player_proficiency + rand::thread_rng().gen_range(-0.2..0.2);
    if ai_proficiency < 0.0 {
        ai_proficiency = 0.0;
    } else if ai_proficiency > 1.0 {
        ai_proficiency = 1.0;
    }
    let ai = Player::new("AI", ai_proficiency);

    // Play the game for a year
    for week in 1..=WEEKS_IN_A_YEAR {
        println!("Week {}", week);

        // Play the game for a week
        for day in 1..=DAYS_IN_A_WEEK {
            let player_score = player.throw_dart();
            let ai_score = ai.throw_dart();

            println!(
                "Day {}: {} scored {} points, {} scored {} points",
                day, player.name, player_score, ai.name, ai_score
            );

            player.cumulative_score += player_score;
            ai.cumulative_score += ai_score;
        }

        // Determine the winner for the week
        if player.cumulative_score > ai.cumulative_score {
            println!("{} wins the week!\n", player.name);
        } else if player.cumulative_score < ai.cumulative_score {
            println!("{} wins the week!\n", ai.name);
        } else {
            println!("It's a tie!\n");
        }

        // Reset cumulative scores for the next week
        player.cumulative_score = 0;
        ai.cumulative_score = 0;
    }  

    println!("Game over! Thanks for playing.");
}

fn get_input(prompt: &str) -> String {
    
}
