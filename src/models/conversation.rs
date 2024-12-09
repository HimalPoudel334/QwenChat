use uuid;

#[derive(Debug, Clone, PartialEq)]
pub struct Conversation {
    id: String,
    title: String,
    client: String,
    server: String,
    messages: Vec<Message>,
}

impl Conversation {
    pub fn new(title: String, client: String, server: String, messages: Vec<Message>) -> Self {
        Self {
            id: uuid::Uuid::new_v4().simple().to_string(),
            title,
            client,
            server,
            messages,
        }
    }

    pub fn default() -> Self {
        Self {
            id: String::default(),
            title: String::default(),
            client: String::default(),
            server: String::default(),
            messages: Vec::new(),
        }
    }

    pub fn new_message(&mut self, message: Message) {
        self.messages.push(message);
    }

    pub fn get_id(&self) -> &str {
        &self.id
    }

    pub fn get_title(&self) -> &str {
        &self.title
    }

    pub fn get_messages(&self) -> &[Message] {
        &self.messages
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Message {
    id: String,
    content: String,
    from_server: bool,
}

impl Message {
    pub fn new(content: String, from_server: bool) -> Self {
        Self {
            id: uuid::Uuid::new_v4().simple().to_string(),
            content,
            from_server,
        }
    }

    pub fn get_id(&self) -> &str {
        &self.id
    }

    pub fn get_content(&self) -> &str {
        &self.content
    }

    pub fn get_from_server(&self) -> bool {
        self.from_server
    }
}
