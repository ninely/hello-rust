use trait_demo::summary;
use crate::trait_demo::summary::Summary;

pub mod trait_demo;

fn main() {
    let tweet = summary::Tweet {
        username: "MyName".to_string(),
        content: "This is a tweet content.".to_string(),
    };

    println!("1 new tweet: {}", tweet.summarize());
}