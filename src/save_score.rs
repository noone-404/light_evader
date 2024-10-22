use std::fs::File;
use std::io::{BufWriter, Write};

pub fn save_score() -> std::io::Result<()> {
    // TODO: Implement saving

    let file = File::create("/Data/high_score/high_score.txt")?;

    let mut writer = BufWriter::new(file);

    writer.write_all("Score".to_string().as_bytes())?;

    writer.flush()?;

    Ok(())
}
