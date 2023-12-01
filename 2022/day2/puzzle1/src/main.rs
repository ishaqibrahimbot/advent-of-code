use std::fs;
use std::collections::HashMap;

fn main() {
    let input = fs::read_to_string("./input.txt").expect("Trouble reading file");
    let games: Vec<&str> = input.split('\n').collect();
    let num_games : i32 = games.len().try_into().unwrap();

    let total_score = simulate_games(games);

    println!("Total games played: {}", num_games);
    println!("Average score: {}", total_score / num_games);
    println!("Total score is: {total_score}");
   
}

fn play_game(opponent: &i32, player: &i32) -> i32 {
    let difference = (player - opponent).abs();

    if difference == 0 {
        println!("Game Drawn");
        3
    } else if difference == 2 {
        if player < opponent {
            println!("Player wins");
            6
        } else {
            println!("Opponent wins");
            0
        }
    } else {
        if player > opponent {
            println!("Player wins");
            6
        } else {
            println!("Opponent wins");
            0
        }
    }
}

fn simulate_games(games: Vec<&str>) -> i32 {

    let mut choice_name_map = HashMap::new();
    choice_name_map.insert("A", "Rock");
    choice_name_map.insert("B", "Paper");
    choice_name_map.insert("C", "Scissor");
    choice_name_map.insert("X", "Rock");
    choice_name_map.insert("Y", "Paper");
    choice_name_map.insert("Z", "Scissor");
    
    let mut choice_index_map: HashMap<String, i32> = HashMap::new();
    choice_index_map.insert(String::from("X"), 1);
    choice_index_map.insert(String::from("Y"), 2);
    choice_index_map.insert(String::from("Z"), 3);
    choice_index_map.insert(String::from("A"), 1);
    choice_index_map.insert(String::from("B"), 2);
    choice_index_map.insert(String::from("C"), 3);

    let mut total_for_all_games = 0;
    for game_state in games {
        println!("{game_state}");
        let choices : Vec<&str> = game_state.split(" ").collect();
        let opponent_choice = choices[0];
        let player_choice = choices[1];
        println!("{} vs {}", choice_name_map.get(opponent_choice).unwrap(), choice_name_map.get(player_choice).unwrap());
        let score = play_game(choice_index_map.get(opponent_choice).unwrap(), choice_index_map.get(player_choice).unwrap());
        println!("game score: {score}");
        let total_score = score + choice_index_map.get(player_choice).unwrap();
        total_for_all_games += total_score;
        println!("total score: {total_score}\n");
    }

    total_for_all_games

}



// rock (1) beats scissor (3)
// paper (2) defeats rock (1)
// scissor (3) defeats paper (2)

// opponent rock 1
// X (you have to lose) --> pick scissor (3)
// Y (you have to draw) --> pick rock (1)
// Z (you have to win) --> pick paper (2)

// opponent paper 2
// X (you have to lose) --> pick rock (1)
// Y (you have to draw) --> pick paper (2)
// Z (win) --> pick scissor (3)

// opponent scissor 3
// X (lose) --> paper 2
// Y (draw) --> scissor 3
// Z (win) --> rock 1