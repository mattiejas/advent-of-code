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

// check syntax returns the invalid token if there is one, and the remaining stack of tokens
fn check_syntax(tokens: &Vec<Token>) -> (Option<(usize, Token)>, Vec<Token>) {
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
                return (Some((i, Token::CloseParen)), stack);
            }

            Token::CloseBracket => {
                if let Some(last) = stack.pop() {
                    if last == Token::OpenBracket {
                        continue;
                    }
                }
                // can't close a bracket without an open one
                return (Some((i, Token::CloseBracket)), stack);
            }

            Token::CloseBrace => {
                if let Some(last) = stack.pop() {
                    if last == Token::OpenBrace {
                        continue;
                    }
                }
                // can't close a brace without an open one
                return (Some((i, Token::CloseBrace)), stack);
            }

            Token::CloseAngular => {
                if let Some(last) = stack.pop() {
                    if last == Token::OpenAngular {
                        continue;
                    }
                }
                // can't close an angular without an open one
                return (Some((i, Token::CloseAngular)), stack);
            }
        }
    }

    return (None, stack);
}

fn get_tokens_list() -> Vec<Vec<Token>> {
    let input = utils::input(10, false);
    return input
        .trim()
        .split('\n')
        .map(|line| tokenize(line))
        .collect();
}

impl day::Day for Day10 {
    fn solve_part1() -> String {
        let tokens_list = get_tokens_list();

        let mut score = 0;
        for tokens in tokens_list {
            let (x, _) = check_syntax(&tokens);
            if let Some((_, token)) = x {
                score += token as isize;
            }
        }

        return score.to_string();
    }

    fn solve_part2() -> String {
        let tokens_list = get_tokens_list();

        let mut scores: Vec<i64> = vec![];
        for tokens in tokens_list {
            let (x, stack) = check_syntax(&tokens);
            if x.is_none() && !stack.is_empty() {
                let mut remaining = stack.clone();
                let mut score = 0;

                while let Some(token) = remaining.pop() {
                    score *= 5;
                    score += match token {
                        Token::OpenParen => 1,
                        Token::OpenBracket => 2,
                        Token::OpenBrace => 3,
                        Token::OpenAngular => 4,
                        _ => 0,
                    };
                }

                scores.push(score);
            }
        }

        scores.sort();
        return scores
            .iter()
            .skip(scores.len() / 2)
            .take(1)
            .collect::<Vec<_>>()[0]
            .to_string();
    }
}
