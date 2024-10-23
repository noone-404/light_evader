use std::fs::File;
use std::io::{BufWriter, Write};

pub fn save_score(score: i64) -> std::io::Result<()> {
    // TODO: Implement saving

    let file = File::create("Data/high_score/high_score.txt")?;

    let mut writer = BufWriter::new(file);

    writer.write_all(format!("High_Score: {}", score).to_string().as_bytes())?;

    writer.flush()?;

    Ok(())
}
