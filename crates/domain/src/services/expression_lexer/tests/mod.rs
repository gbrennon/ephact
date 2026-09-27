use super::*;

impl<'a> ExpressionLexer<'a> {
    fn lex_all_for_test(input: &'a str) -> Result<Vec<ExpressionToken>, LexerError> {
        let mut lexer = ExpressionLexer::new(input);
        let mut tokens = Vec::new();
        loop {
            let token = lexer.next_token()?;
            let done = matches!(token, ExpressionToken::EndOfInput);
            tokens.push(token);
            if done {
                break;
            }
        }
        Ok(tokens)
    }
}

mod errors;
mod identifiers;
mod lexer_flow;
mod numbers;
mod operators;
mod strings;
