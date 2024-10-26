use std::fs::File; // Make sure to tell the system we want to search through the file system
use std::io::{BufWriter, Write}; // We gain the ability to write on files

pub fn _save_score(score: i64) -> std::io::Result<()> {
    let file = File::create("Data/high_score.scs")?; // We create the high score file on the Data folder and also scs because score script

    let mut writer = BufWriter::new(file); // We create the writer for the file

    writer.write_all(format!("High_Score: {}", score).to_string().as_bytes())?; // We write the high score on the file that we passed onto this function

    writer.flush()?; // We flush the writer to it would apply the changes or something I don't know

    Ok(()) // We say it's ok to move on
}
