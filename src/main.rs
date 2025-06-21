mod character;
mod monster;
mod player;


#[derive(Clone)]
struct GameScore {
    player_name: String,
    score: u32,
}


#[derive(Clone, Copy)]
struct SimpleScore {
    score: u32
}

fn main() {
    let gameScore = GameScore {
        player_name: String::from("Player1"),
        score: 100,
    };
    let cloneGameScore = gameScore.clone();
    println!("Original Player Name: {}, Score: {}", gameScore.player_name, gameScore.score);
    println!("Player Name: {}, Score: {}", cloneGameScore.player_name, cloneGameScore.score);

    let s1 = SimpleScore { score: 50 };
    let s2 = s1; 
    println!("SimpleScore s1: {}, s2: {}", s1.score, s2.score);
}