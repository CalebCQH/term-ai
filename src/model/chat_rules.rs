pub enum ChatRules {
    User,
    Assistant,
}

impl ChatRules {
    pub fn display_name(&self) -> String {
        match self {
            ChatRules::User => "user".to_string(),
            ChatRules::Assistant => "assistant".to_string(),
        }
    }
}
