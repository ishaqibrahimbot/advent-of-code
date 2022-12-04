use std::fs;
use std::collections::HashMap;
use std::hash::Hash;

fn main() {
    let input = fs::read_to_string("./input.txt").expect("Trouble reading file");
    let games: Vec<&str> = input.split('\n').collect();
    let num_games : i32 = games.len().try_into().unwrap();

    let total_score = simulate_games(games);

    println!("Total games played: {}", num_games);
    println!("Average score: {}", total_score / num_games);
    println!("Total score is: {total_score}");
   
}

// opponent rock 1
// X (you have to lose) --> pick scissor (3) C
// Y (you have to draw) --> pick rock (1) A
// Z (you have to win) --> pick paper (2) B

// opponent paper 2
// X (you have to lose) --> pick rock (1) A
// Y (you have to draw) --> pick paper (2) B
// Z (win) --> pick scissor (3) C

// opponent scissor 3
// X (lose) --> paper 2 B
// Y (draw) --> scissor 3 C
// Z (win) --> rock 1 A

fn get_choice_for_outcome(opponent: &i32, outcome: &str) -> String {
    let rock = String::from("A");
    let paper = String::from("B");
    let scissor = String::from("C");
    if *opponent == 1 {
        if outcome == "X" {
            scissor
        } else if outcome == "Y" {
            rock
        } else {
            paper
        }
    } else if *opponent == 2 {
        if outcome == "X" {
            rock
        } else if outcome == "Y" {
            paper
        } else {
            scissor
        }
    } else {
        if outcome == "X" {
            paper
        } else if outcome == "Y" {
            scissor
        } else {
            rock
        }
    }
}

fn simulate_games(games: Vec<&str>) -> i32 {

    let mut semantic_meaning_map = HashMap::new();
    semantic_meaning_map.insert("A", "Rock");
    semantic_meaning_map.insert("B", "Paper");
    semantic_meaning_map.insert("C", "Scissor");
    semantic_meaning_map.insert("X", "Lose");
    semantic_meaning_map.insert("Y", "Draw");
    semantic_meaning_map.insert("Z", "Win");
    
    let mut choice_index_map: HashMap<String, i32> = HashMap::new();
    choice_index_map.insert(String::from("A"), 1);
    choice_index_map.insert(String::from("B"), 2);
    choice_index_map.insert(String::from("C"), 3);

    let mut game_score_map = HashMap::new();
    game_score_map.insert("X", 0);
    game_score_map.insert("Y", 3);
    game_score_map.insert("Z", 6);

    let mut total_for_all_games = 0;

    for game_state in games {
        println!("{game_state}");
        let game_state_vec : Vec<&str> = game_state.split(" ").collect();
        let opponent_choice = game_state_vec[0];
        let desired_outcome = game_state_vec[1];
        let required_choice = get_choice_for_outcome(choice_index_map.get(opponent_choice).unwrap(), desired_outcome);
        println!("Opponent played: {}", semantic_meaning_map.get(opponent_choice).unwrap());
        println!("Need to {} game", semantic_meaning_map.get(desired_outcome).unwrap());
        println!("Choice should be {}", semantic_meaning_map.get(&required_choice[..]).unwrap());

        let game_outcome_score = game_score_map.get(desired_outcome).unwrap();
        let required_choice_score = choice_index_map.get(&required_choice).unwrap();

        println!("Game score: {game_outcome_score}");
        println!("Choice score: {required_choice_score}");

        let total_score = game_outcome_score + required_choice_score;
        total_for_all_games += total_score;

        println!("total score: {total_score}\n");
    }

    total_for_all_games

}



// rock (1) beats scissor (3)
// paper (2) defeats rock (1)
// scissor (3) defeats paper (2)

