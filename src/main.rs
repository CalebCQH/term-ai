use std::env;
use clap::Parser;
mod config;
mod api;
mod ui;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    api_key: Option<String>,
}

fn main() {
    let args = Args::parse();

    let api_key = args
        .api_key
        .or_else(|| env::var("OPENAI_API_KEY").ok())
        .unwrap_or_else(|| {
            println!("请输入 API Key:");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            input.trim().to_string()
        });

    let config = config::Config {
        api_key: api_key,
    };
    let response = api::send_message();
    println!("{:?}", response);
}
