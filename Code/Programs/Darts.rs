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

    fn adjust_proficiency(&mut self, rounds_left: usize) {
        // Decrease proficiency based on how far into the tournament or league the player loses
        if rounds_left > 0 {
            let decrease_factor = 0.1 * rounds_left as f64;
            self.proficiency = (self.proficiency - decrease_factor).max(0.0);
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

        // Calculate and display scores
        for player in &mut self.players {
            player.calculate_average_score(self.rounds as f64);
        }

        println!("Round {} Scores:", round_number);
        for player in &self.players {
            println!(
                "{}: Total Score - {}, Average Score - {:.2}",
                player.name, player.total_score, player.average_score
            );
        }
        println!();

        // Determine winners and adjust proficiency
        let winners = self.determine_winners();
        for player in &mut self.players {
            if !winners.contains(&player.name) {
                player.adjust_proficiency(self.rounds - round_number);
            }
        }
    }
    
    fn determine_winner(&self) -> Vec<String> {
        let max_score = self.players.iter().map(|p| p.total_score).max().unwrap_or(0);
        
        self.players
            .iter()
            .filter(|player| player.total_score == max_score)
            .map(|player| player.name.clone())
            .collect()   
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
                game.play_round(index + 1);
        }

        println!("Tournament Over!");
        
        // Display final proficiency after the tournament
        println!("Final Proficiency Levels:");
        for player in &self.games[0].players {
            println!("{}: Proficiency - {:.2}", player.name, player.proficiency);
        }
        println!();
    }
}

#[derive(Debug, Clone)]
struct League {
    players: Vec<Player>,
    games: Vec<Game>,
}

impl League {
    fn new(players: Vec<Player>, games: Vec<Game>) -> Self {
        League { players, games }    
    }  

    fn play_league(&mut self) {
        for (index, game) in self.games.iter_mut().enumerate() {
            println!("Game {} of the league", index + 1);
            game.play_round(index + 1);
        }  

        println!("League Over!");
        
        // Display final proficiency after the league
        println!("Final Proficiency Levels:");
        for player in &self.games[0].players {
            println!("{}: Proficiency - {:.2}", player.name, player.proficiency);
        }
        println!();
    }
}

fn main() {
    let player1 = Player::new("Player 1", 0.8);
    let player2 = Player::new("Player 2", 0.7);
    let player3 = Player::new("Player 3", 0.6);  

    let game1 = Game::new(vec![player1.clone(), player2.clone(), player3.clone()], 3);
    let game2 = Game::new(vec![player1.clone(), player2.clone(), player3.clone()], 3);
    let game3 = Game::new(vec![player1.clone(), player2.clone(), player3.clone()], 3);

    let tournament = Tournament::new(vec![game1.clone(), game2.clone(), game3.clone()]);
    let mut tournament_manager = tournament;
    tournament_manager.play_tournament();

    let league = League::new(vec![player1.clone(), player2.clone(), player3.clone()], vec![game1, game2, game3]);
    let mut league_manager = league;
    league_manager.play_league();
}
