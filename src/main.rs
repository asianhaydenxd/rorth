mod lex;

fn run(file_name: String, code: String) {
    let tokens = crate::lex::lex(file_name, code);
}

fn main() {
    run("main".to_string(), "test".to_string());
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use crate::lex::*;

    #[test]
    fn test() {
        assert_eq!(
            lex("main".to_string(), "test".to_string()),
            vec![Token::Identifier(
                "test".to_string(),
                LexContext {
                    index: 0,
                    line: 0,
                    col: 0,
                    file_name: "main".to_string(),
                    file_text: "test".to_string(),
                }
            )]
        );
    }
}
