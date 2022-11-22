#[derive(PartialEq, Eq, Clone, Debug)]
pub struct LexContext {
    pub index: usize,
    pub line: usize,
    pub col: usize,
    pub file_name: String,
    pub file_text: String,
}

impl LexContext {
    fn advance(&mut self) {
        self.col += 1;
        if self.file_text.chars().nth(self.index) == Some('\n') {
            self.line += 1;
            self.col = 0;
        }
        self.index += 1;
    }

    fn has_valid_index(&mut self) -> bool {
        self.index < self.file_text.len()
    }

    fn current_character(&self) -> Option<char> {
        self.file_text.chars().nth(self.index)
    }

    fn new(file_name: String, file_text: String) -> LexContext {
        LexContext {
            index: 0,
            line: 0,
            col: 0,
            file_name,
            file_text,
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
pub enum Token {
    Identifier(String, LexContext),
}

pub fn lex(file_name: String, code: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut context = LexContext::new(file_name, code);

    while context.has_valid_index() {
        if let Some(character) = context.current_character() {
            if character.is_alphabetic() {
                tokens.push(lex_word(&mut context));
            }
        }
        context.advance();
    }
    tokens
}

fn lex_word(context: &mut LexContext) -> Token {
    let token_context = context.clone();
    let mut word: String = String::new();

    while let Some(character) = context.current_character() {
        if !character.is_alphanumeric() {
            break;
        }
        word.push(character);
        context.advance();
    }
    Token::Identifier(word.clone(), token_context)
}
