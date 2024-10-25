use std::fs::File;
use std::io::Read;

mod save_score;

pub fn return_high_score() -> i64 {
    let path = "Data/high_score/high_score.scs";

    let mut file = File::open(path).unwrap();
        
    // Read the file
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    // Store where the data is
    let data = contents.trim();

    let high_score = data.split("High_Score: ");

    let high_score = high_score.last().unwrap().parse::<i64>().unwrap();

    return high_score;
}

pub fn compare_high_score(score: i64) {
    let high_score = return_high_score();

    if score > high_score {
        println!("New High Score: {}", score);
        save_score::save_score(score).unwrap();
    } else {
        return;
    }
}
