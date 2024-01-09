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

        println!("Tournament Over!");

        let mut overall_scores: HashMap<String, f64> = HashMap::new();

        for game in &self.games {
            for player in &game.players {
                let entry = overall_scores.entry(player.name.clone()).or_insert(0.0);
                *entry += player.average_score;
            }
        }

        let sorted_scores: BTreeMap<_, _> = overall_scores.into_iter().collect();

        println!("Overall Tournament Standings:");
        for (name, avg_score) in sorted_scores.iter().rev() {
            println!("{}: Average Score - {:.2}", name, avg_score);
        }

        let winner = sorted_scores.iter().next().map(|(name, _)| name.clone()).unwrap();
        println!("Tournament Winner: {}\n", winner);
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
            game.play_round();
            game.display_scores();
            game.calculate_average_scores();
        }  

        println!("League Over!");

        let mut overall_scores: HashMap<String, f64> = HashMap::new();

        for player in &self.players {
            overall_scores.insert(player.name.clone(), player.average_score);
        }

        let sorted_scores: BTreeMap<_, _> = overall_scores.into_iter().collect();

        println!("Overall League Standings:");
        for (name, avg_score) in sorted_scores.iter().rev() {
            println!("{}: Average Score - {:.2}", name, avg_score);
        }

        let winner = sorted_scores.iter().next().map(|(name, _)| name.clone()).unwrap();
        println!("League Winner: {}\n", winner);
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
