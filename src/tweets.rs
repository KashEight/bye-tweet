use std::{fs::File, io::BufReader, path::Path};

use egg_mode::Token;
use log::{info, warn};
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(transparent)]
pub struct TweetData {
    data: Vec<TweetObject>,
}

impl TweetData {
    pub fn read_from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        let tweets = serde_json::from_reader(reader)?;

        Ok(tweets)
    }

    pub async fn delete_tweet(&self, token: &Token) {
        for tweet_obj in &self.data {
            let id = &tweet_obj.tweet.id;
            info!("deleting tweet_id: {}", id);
            if let Err(err) = egg_mode::tweet::delete(id.parse::<u64>().unwrap(), token).await {
                warn!("{}", err);
            };
        }
    }
}

#[derive(Debug, Deserialize)]
struct TweetObject {
    tweet: Tweet,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Tweet {
    id: String,
    created_at: String,
}
