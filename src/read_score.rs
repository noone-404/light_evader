use std::fs::File;
use std::io::Read;

#[path = "save_score.rs"]
mod save_score;

pub fn return_high_score() -> i64 {
    let path = "Data/high_score.scs";

    let mut file = File::open(path).unwrap();

    // Read the file
    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();

    drop(file);

    // Store where the data is
    let data = contents.trim();

    let high_score = data.split("High_Score: ");

    let high_score = high_score.last().unwrap().parse::<i64>().unwrap();

    return high_score;
}

pub fn new_high_score(score: i64) -> bool {
    let high_score = return_high_score();

    if score > high_score {
        save_score::_save_score(score).unwrap();

        return true;
    } else {
        return false;
    }
}
