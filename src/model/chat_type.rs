pub enum ChatTypeEnum {
    Text,
    Thinking,
    Image,
}

impl ChatTypeEnum {
    pub fn display(&self) -> String {
        match self {
            ChatTypeEnum::Text => "text".to_string(),
            ChatTypeEnum::Thinking => "thinking".to_string(),
            ChatTypeEnum::Image => "image".to_string(),
        }
    }
}
