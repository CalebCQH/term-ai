pub enum ChatTypeEnum {
    Text,
}

impl ChatTypeEnum {
    pub fn display(&self) -> String {
        match self {
            ChatTypeEnum::Text => "text".to_string(),
        }
    }
}
