use std::{env};

use bye_tweet::read::{self};
use log::{info, warn};

#[tokio::main]
async fn main() {
    env_logger::init();
    let path = env::var("DATA_PATH").expect("No `DATA_PATH` in env");
    let consumer_key = env::var("COMSUMER_KEY").expect("No `CONSUMER_KEY` in env");
    let consumer_key_secret =
        env::var("COMSUMER_KEY_SECRET").expect("No `CONSUMER_KEY_SECRET` in env");
    let access_token = env::var("ACCESS_TOKEN").expect("No `ACCESS_TOKEN` in env");
    let access_token_secret =
        env::var("ACCESS_TOKEN_SECRET").expect("No `ACCESS_TOKEN_SECRET` in env");

    let con_token = egg_mode::KeyPair::new(consumer_key, consumer_key_secret);

    let access_token = egg_mode::KeyPair::new(access_token, access_token_secret);
    let token = egg_mode::Token::Access {
        consumer: con_token,
        access: access_token,
    };

    let tw_data = read::read_and_deserialize(path).unwrap_or_else(|err| panic!("{}", err));

    let ids = tw_data
        .into_iter()
        .filter_map(|tw| {
            let splitted_tw = tw
                .tweet
                .created_at
                .split_whitespace()
                .collect::<Vec<&str>>();
            if splitted_tw[5].parse::<u32>().unwrap() < 2019
                || (splitted_tw[5].parse::<u32>().unwrap() == 2019 && (splitted_tw[1] == "Jan" || splitted_tw[1] == "Feb" || splitted_tw[1] == "Mar"))
            {
                Some(tw.tweet.id.parse::<u64>().unwrap())
            } else {
                None
            }
        })
        .collect::<Vec<u64>>();

    for id in ids {
        info!("deleting {}", id);
        if let Err(err) = egg_mode::tweet::delete(id, &token).await {
            warn!("{}", err);
        };
    }
}
