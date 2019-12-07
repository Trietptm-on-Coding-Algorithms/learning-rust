use async_std;
use num_format::{Locale, ToFormattedString};

async fn hello() {
    let answer = 1_234_5;
    println!("Hello, {}", answer.to_formatted_string(&Locale::en));
}

#[async_std::main]
async fn main() {
    hello().await;
}
