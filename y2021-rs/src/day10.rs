use crate::{day, utils};

pub struct Day10 {}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Token {
    OpenParen,
    CloseParen = 3,
    OpenBracket,
    CloseBracket = 57,
    OpenBrace,
    CloseBrace = 1197,
    OpenAngular,
    CloseAngular = 25137,
}

fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    for c in input.chars() {
        match c {
            '(' => tokens.push(Token::OpenParen),
            ')' => tokens.push(Token::CloseParen),
            '[' => tokens.push(Token::OpenBracket),
            ']' => tokens.push(Token::CloseBracket),
            '{' => tokens.push(Token::OpenBrace),
            '}' => tokens.push(Token::CloseBrace),
            '<' => tokens.push(Token::OpenAngular),
            '>' => tokens.push(Token::CloseAngular),
            _ => {}
        }
    }
    tokens
}

// check syntax returns the invalid token if there is one
fn check_syntax(tokens: &Vec<Token>) -> Option<(usize, Token)> {
    let mut stack = Vec::new();

    for (i, token) in tokens.iter().cloned().enumerate() {
        match token {
            Token::OpenParen => stack.push(Token::OpenParen),
            Token::OpenBracket => stack.push(Token::OpenBracket),
            Token::OpenBrace => stack.push(Token::OpenBrace),
            Token::OpenAngular => stack.push(Token::OpenAngular),

            // closing tokens
            Token::CloseParen => {
                if let Some(last) = stack.pop() {
                    if last == Token::OpenParen {
                        continue;
                    }
                }
                // can't close a paren without an open one
                return Some((i, Token::CloseParen));
            }

            Token::CloseBracket => {
                if let Some(last) = stack.pop() {
                    if last == Token::OpenBracket {
                        continue;
                    }
                }
                // can't close a bracket without an open one
                return Some((i, Token::CloseBracket));
            }

            Token::CloseBrace => {
                if let Some(last) = stack.pop() {
                    if last == Token::OpenBrace {
                        continue;
                    }
                }
                // can't close a brace without an open one
                return Some((i, Token::CloseBrace));
            }

            Token::CloseAngular => {
                if let Some(last) = stack.pop() {
                    if last == Token::OpenAngular {
                        continue;
                    }
                }
                // can't close an angular without an open one
                return Some((i, Token::CloseAngular));
            }
        }
    }

    return None;
}

impl day::Day for Day10 {
    fn solve_part1() -> String {
        let input = utils::input(10, false);
        let token_list: Vec<Vec<Token>> = input
            .trim()
            .split('\n')
            .map(|line| tokenize(line))
            .collect();

        let mut score = 0;
        for token_list in token_list {
            if let Some((_, token)) = check_syntax(&token_list) {
                score += token as isize;
            }
        }

        return score.to_string();
    }

    fn solve_part2() -> String {
        todo!()
    }
}
