mod lex;

fn run(file_name: String, code: String) {
    let _tokens = crate::lex::lex(file_name, code);
}

fn main() {
    run("main".to_string(), "test".to_string());
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use crate::lex::*;

    fn test_lexer(code: &str, target: Vec<&str>) {
        let tokens: Vec<Token> = lex("test".to_string(), code.to_string());
        for (token, target_string) in tokens.iter().zip(target.iter()) {
            match token {
                Token::Identifier(name, _) |
                Token::Number(name, _) => {
                    assert_eq!(*name, target_string.to_string(),
                        "token names \"{}\" and \"{}\" do not match; output tokens are {:?}",
                        *name, target_string.to_string(), tokens);
                },
            }
        }
        assert_eq!(tokens.len(), target.len(),
            "token lengths {} and {} do not match; output tokens are {:?}",
            tokens.len(), target.len(), tokens);
    }

    #[test]
    fn test() {
        test_lexer("test", vec!["test"]);
    }

    #[test]
    fn number_lex_test() {
        test_lexer("1 2 3.0", vec!["1", "2", "3.0"])
    }

    #[test]
    fn symbol_lex_test() {
        test_lexer("1 +", vec!["1", "+"]);
    }

}
