mod config;
mod api;
mod ui;
mod model;
mod result;

#[tokio::main]
async fn main() {
    
    let chat = {
            println!("请输入对话:");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            input.trim().to_string()
        };
    let config = config::api_config::Config {
        // id: "1".to_string(),
        model: "MiniMax-M2.7".to_string(),
        post_url: "https://api.minimaxi.com/v1/chat/completions".to_string(),
        api_key: "sk-cp-w1kmO3saYZ9KPgz-gz7o0CRsA-OkHXNmtmF1y2SE6CdnW2RZQ_secwWl_CXDx9ssWpcafIIhZ-pNN35CJ1IKwYMRuYotuQbERfzlHfP6Ai53COeS2m4Urig".to_string(),
    };
    match api::send_message(&chat, &config).await {
        result::ApiResponse::Success(response) => {
            let content = &response.choices[0].message.content;
            println!("{}", content)
        }
        result::ApiResponse::Error(e) => eprintln!("Error: {}", e.message),
    }
}
