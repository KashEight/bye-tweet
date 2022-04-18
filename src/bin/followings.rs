use std::{env, process::exit};

use bye_tweet::followings::FollowingData;
use log::error;

#[tokio::main]
async fn main() {
    env_logger::init();
    let mode = if let Ok(v) = env::var("MODE") {
        v
    } else {
        "block".to_owned()
    };
    let path = env::var("DATA_PATH").expect("No `DATA_PATH` in env");
    let consumer_key = env::var("CONSUMER_KEY").expect("No `CONSUMER_KEY` in env");
    let consumer_key_secret =
        env::var("CONSUMER_KEY_SECRET").expect("No `CONSUMER_KEY_SECRET` in env");
    let access_token = env::var("ACCESS_TOKEN").expect("No `ACCESS_TOKEN` in env");
    let access_token_secret =
        env::var("ACCESS_TOKEN_SECRET").expect("No `ACCESS_TOKEN_SECRET` in env");

    let con_token = egg_mode::KeyPair::new(consumer_key, consumer_key_secret);

    let access_token = egg_mode::KeyPair::new(access_token, access_token_secret);
    let token = egg_mode::Token::Access {
        consumer: con_token,
        access: access_token,
    };

    let following_data = FollowingData::read_from_file(path).unwrap_or_else(|err| {
        error!("{}", err);
        exit(1);
    });

    match mode.as_str() {
        "unfollow" => following_data.unfollow(&token).await,
        "block" => following_data.resolve_follow(&token).await,
        _ => exit(0),
    }
}
