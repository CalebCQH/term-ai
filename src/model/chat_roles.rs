pub enum ChatRoles {
    User,
    Assistant,
}

impl ChatRoles {
    pub fn display_name(&self) -> String {
        match self {
            ChatRoles::User => "user".to_string(),
            ChatRoles::Assistant => "assistant".to_string(),
        }
    }
}
