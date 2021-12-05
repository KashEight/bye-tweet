use std::{fs::File, io::BufReader, path::Path};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TweetData {
    pub tweet: Tweet,
}

#[derive(Debug, Deserialize)]
pub struct Tweet {
    pub id: String,
    pub created_at: String,
}

pub fn read_and_deserialize<P: AsRef<Path>>(
    path: P,
) -> Result<Box<Vec<TweetData>>, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let tweets = serde_json::from_reader(reader)?;

    Ok(tweets)
}
