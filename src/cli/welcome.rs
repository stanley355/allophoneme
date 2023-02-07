const WELCOME_TEXTS: [&str; 4] = [
    "Welcome Boss!",
    "How can I help you?",
    "1. Read Excel",
    "2. Migrate Excel",
];

pub fn welcome_text() {
    for text in WELCOME_TEXTS {
        println!("{}", text);
    }
}
