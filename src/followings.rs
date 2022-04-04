use std::{fs::File, io::BufReader, path::Path};

use egg_mode::Token;
use log::{info, warn};
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(transparent)]
pub struct FollowingData {
    data: Vec<FollowingObject>,
}

impl FollowingData {
    pub fn read_from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        let tweets = serde_json::from_reader(reader)?;

        Ok(tweets)
    }

    pub async fn unfollow(&self, token: &Token) {
        for following_obj in &self.data {
            let account_id = &following_obj.following.accountId;
            info!("unfollowing account_id: {}", account_id);
            if let Err(err) = egg_mode::user::unfollow(account_id.clone(), token).await {
                warn!("{}", err)
            }
        }
    }

    pub async fn resolve_follow(&self, token: &Token) {
        for following_obj in &self.data {
            let account_id = &following_obj.following.accountId;
            info!("blocking account_id: {}", account_id);
            if let Err(err) = egg_mode::user::block(account_id.clone(), token).await {
                warn!("{}", err);
                continue;
            }
            info!("unblocking account_id: {}", account_id);
            if let Err(err) = egg_mode::user::unblock(account_id.clone(), token).await {
                warn!("{}", err);
            }
        }
    }
}

#[derive(Deserialize)]
struct FollowingObject {
    following: Following,
}

#[allow(non_snake_case)]
#[derive(Deserialize)]
struct Following {
    accountId: String,
}
