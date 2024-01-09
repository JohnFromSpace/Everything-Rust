use rand::Rng;
use std::collections::{HashMap, BTreeMap};
use std::io;

#[derive(Debug, Clone)]
struct Player {
    name: String,
    proficiency: f64,
    total_score: usize,
    average_score: f64,
}

impl Player {
    fn new(name: &str, proficiency: f64) -> Self {
        Player {
            name: name.to_string(),
            proficiency,
            total_score: 0,
            average_score: 0.0,
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

#[derive(Debug, Clone)]
struct Game {
    players: Vec<Player>,
    rounds: usize,
}

impl Game {
    fn new(players: Vec<Player>, rounds: usize) -> Self {
        Game { players, rounds }    
    }    

    fn play_round(&mut self) {
        for _ in 0..self.rounds {
            for player in &mut self.players {
                let score = player.throw_dart();
                println!("{} scored {} points", player.name, score);
                player.total_score += score;
            }
        }    
    }

    fn calculate_average_scores(&mut self) {
        for player in &mut self.players {
            player.average_score = (player.total_score as f64) / (self.rounds as f64);
        }
    }

    fn display_scores(&self) {
        println!("Current Scores:");
        for player in &self.players {
            println!(
                "{}: Total Score - {}, Average Score - {:.2}",
                player.name, player.total_score, player.average_score
            );
        }
        println!();    
    }

    fn determine_winner(&self) -> Vec<String> {
        let mut winners = Vec::new();
        let max_score = self.players.iter().map(|p| p.total_score).max().unwrap_or(0);

        for player in &self.players {
            if player.total_score == max_score {
                winners.push(player.name.clone());
            }
        }

        winners        
    }
}

#[derive(Debug, Clone)]
struct Tournament {
    games: Vec<Game>,
}

impl Tournament {
    fn new(games: Vec<Game>) -> Self {
        Tournament { games }    
    } 

   fn play_tournament(&mut self) {
       for (index, game) in self.games.iter_mut().enumerate() {
            println!("Game {} of the tournament", index + 1);
            game.play_round();
            game.display_scores();
            game.calculate_average_scores();
        }    
   }
    
}
