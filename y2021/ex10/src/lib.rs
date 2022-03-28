use std::{
    collections::VecDeque,
    fmt::{Display, Write},
};

use thiserror::Error;

#[derive(Error, Debug)]
enum ParseErr {
    #[error("Expression incomplete")]
    Incomplete(VecDeque<Token>),
    #[error("Expression corrupted: expected {0}, but found {1} instead")]
    Corrupted(Token, Token),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Token {
    OPEN1,  // (
    CLOSE1, // )
    OPEN2,  // [
    CLOSE2, // ]
    OPEN3,  // {
    CLOSE3, // }
    OPEN4,  // <
    CLOSE4, // >
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Token::*;
        f.write_char(match self {
            OPEN1 => '(',
            CLOSE1 => ')',
            OPEN2 => '[',
            CLOSE2 => ']',
            OPEN3 => '{',
            CLOSE3 => '}',
            OPEN4 => '<',
            CLOSE4 => '>',
        })
    }
}

impl From<char> for Token {
    fn from(c: char) -> Self {
        use Token::*;
        match c {
            '(' => OPEN1,
            ')' => CLOSE1,
            '[' => OPEN2,
            ']' => CLOSE2,
            '{' => OPEN3,
            '}' => CLOSE3,
            '<' => OPEN4,
            '>' => CLOSE4,
            _ => unreachable!("Invalid char"),
        }
    }
}

impl Token {
    fn matching(&self) -> Self {
        use Token::*;
        match self {
            OPEN1 => CLOSE1,
            CLOSE1 => OPEN1,
            OPEN2 => CLOSE2,
            CLOSE2 => OPEN2,
            OPEN3 => CLOSE3,
            CLOSE3 => OPEN3,
            OPEN4 => CLOSE4,
            CLOSE4 => OPEN4,
        }
    }

    fn matches(&self, token: Token) -> bool {
        self.matching() == token
    }

    fn is_open(&self) -> bool {
        use Token::*;
        matches!(self, OPEN1 | OPEN2 | OPEN3 | OPEN4)
    }

    fn is_close(&self) -> bool {
        !self.is_open()
    }

    fn error_score(&self) -> Option<usize> {
        use Token::*;
        match self {
            CLOSE1 => Some(3),
            CLOSE2 => Some(57),
            CLOSE3 => Some(1197),
            CLOSE4 => Some(25137),
            _ => None,
        }
    }

    fn autocomplete_score(&self) -> Option<usize> {
        use Token::*;
        match self {
            CLOSE1 => Some(1),
            CLOSE2 => Some(2),
            CLOSE3 => Some(3),
            CLOSE4 => Some(4),
            _ => None,
        }
    }
}

fn parse_expr(line: &str) -> Result<Vec<Token>, ParseErr> {
    let mut stack: VecDeque<Token> = Default::default();
    let mut tokens: Vec<Token> = vec![];

    for c in line.chars() {
        let current_token: Token = c.into();

        tokens.push(current_token);
        if let Some(expected) = stack.pop_back() {
            if current_token.is_close() {
                if !current_token.matches(expected) {
                    // closing token does not match previous token in the stack
                    return Err(ParseErr::Corrupted(expected.matching(), current_token));
                }
                // matching brackets, remove from the stack and don't add current one
            } else {
                // re-add token to the top of the stack and add new token as well
                stack.push_back(expected);
                stack.push_back(current_token);
            }
        } else {
            // add current token to the stack
            stack.push_back(current_token);
        }
    }

    if !stack.is_empty() {
        return Err(ParseErr::Incomplete(stack));
    }

    Ok(tokens)
}

fn build_completion_seq(stack: &mut VecDeque<Token>) -> Vec<Token> {
    let mut completion_seq: Vec<Token> = vec![];

    while !stack.is_empty() {
        completion_seq.push(stack.pop_back().unwrap().matching());
    }

    completion_seq
}

pub fn part1(input: &str) -> usize {
    input
        .lines()
        .filter_map(|line| {
            let result = parse_expr(line);
            if let Err(ParseErr::Corrupted(_, found)) = result {
                return found.error_score();
            }

            None
        })
        .sum()
}

pub fn part2(input: &str) -> usize {
    let mut autocomplete_scores: Vec<usize> = input
        .lines()
        .filter_map(|line| {
            let mut result = parse_expr(line);
            if let Err(ParseErr::Incomplete(ref mut stack)) = result {
                let completion_seq = build_completion_seq(stack);
                let autocomplete_score = completion_seq.iter().fold(0_usize, |acc, current| {
                    (acc * 5) + current.autocomplete_score().unwrap()
                });
                return Some(autocomplete_score);
            }

            None
        })
        .collect();

    // the score in the middle is the final result
    let nth = autocomplete_scores.len() / 2;
    let (_, median, _) = autocomplete_scores.select_nth_unstable(nth);
    *median
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_expr() {
        // {([(<{}[<>[]}>{[]{[(<()> - Expected ], but found } instead.
        let expr = "{([(<{}[<>[]}>{[]{[(<()>";
        let result = parse_expr(expr);
        assert!(matches!(
            result,
            Err(ParseErr::Corrupted(Token::CLOSE2, Token::CLOSE3))
        ));

        // [[<[([]))<([[{}[[()]]] - Expected ], but found ) instead.
        let expr = "[[<[([]))<([[{}[[()]]]";
        let result = parse_expr(expr);
        assert!(matches!(
            result,
            Err(ParseErr::Corrupted(Token::CLOSE2, Token::CLOSE1))
        ));

        // [{[{({}]{}}([{[{{{}}([] - Expected ), but found ] instead.
        let expr = "[{[{({}]{}}([{[{{{}}([]";
        let result = parse_expr(expr);
        assert!(matches!(
            result,
            Err(ParseErr::Corrupted(Token::CLOSE1, Token::CLOSE2))
        ));

        // [<(<(<(<{}))><([]([]() - Expected >, but found ) instead.
        let expr = "[<(<(<(<{}))><([]([]()";
        let result = parse_expr(expr);
        assert!(matches!(
            result,
            Err(ParseErr::Corrupted(Token::CLOSE4, Token::CLOSE1))
        ));

        // <{([([[(<>()){}]>(<<{{ - Expected ], but found > instead.
        let expr = "<{([([[(<>()){}]>(<<{{";
        let result = parse_expr(expr);
        assert!(matches!(
            result,
            Err(ParseErr::Corrupted(Token::CLOSE2, Token::CLOSE4))
        ));

        // [({(<(())[]>[[{[]{<()<>> - incomplete
        let expr = "[({(<(())[]>[[{[]{<()<>>";
        let result = parse_expr(expr);
        assert!(matches!(result, Err(ParseErr::Incomplete(_))));

        // [({(<(())[]>[[{[]{<()<>>}}]])})] - complete
        let expr = "[({(<(())[]>[[{[]{<()<>>}}]])})]";
        let result = parse_expr(expr);
        assert!(matches!(result, Ok(_)));
    }

    #[test]
    fn test_build_completion_seq() {
        // [({(<(())[]>[[{[]{<()<>> - Complete by adding }}]])})].
        let line = "[({(<(())[]>[[{[]{<()<>>";
        let expected: Vec<Token> = "}}]])})]".chars().map(|t| t.into()).collect();
        let mut result = parse_expr(line);
        assert!(matches!(result, Err(ParseErr::Incomplete(_))));
        if let Err(ParseErr::Incomplete(ref mut stack)) = result {
            assert_eq!(build_completion_seq(stack), expected);
        }

        // [(()[<>])]({[<{<<[]>>( - Complete by adding )}>]}).
        let line = "[(()[<>])]({[<{<<[]>>(";
        let expected: Vec<Token> = ")}>]})".chars().map(|t| t.into()).collect();
        let mut result = parse_expr(line);
        assert!(matches!(result, Err(ParseErr::Incomplete(_))));
        if let Err(ParseErr::Incomplete(ref mut stack)) = result {
            assert_eq!(build_completion_seq(stack), expected);
        }

        // (((({<>}<{<{<>}{[]{[]{} - Complete by adding }}>}>)))).
        let line = "(((({<>}<{<{<>}{[]{[]{}";
        let expected: Vec<Token> = "}}>}>))))".chars().map(|t| t.into()).collect();
        let mut result = parse_expr(line);
        assert!(matches!(result, Err(ParseErr::Incomplete(_))));
        if let Err(ParseErr::Incomplete(ref mut stack)) = result {
            assert_eq!(build_completion_seq(stack), expected);
        }

        // {<[[]]>}<{[{[{[]{()[[[] - Complete by adding ]]}}]}]}>.
        let line = "{<[[]]>}<{[{[{[]{()[[[]";
        let expected: Vec<Token> = "]]}}]}]}>".chars().map(|t| t.into()).collect();
        let mut result = parse_expr(line);
        assert!(matches!(result, Err(ParseErr::Incomplete(_))));
        if let Err(ParseErr::Incomplete(ref mut stack)) = result {
            assert_eq!(build_completion_seq(stack), expected);
        }

        // <{([{{}}[<[[[<>{}]]]>[]] - Complete by adding ])}>.
        let line = "<{([{{}}[<[[[<>{}]]]>[]]";
        let expected: Vec<Token> = "])}>".chars().map(|t| t.into()).collect();
        let mut result = parse_expr(line);
        assert!(matches!(result, Err(ParseErr::Incomplete(_))));
        if let Err(ParseErr::Incomplete(ref mut stack)) = result {
            assert_eq!(build_completion_seq(stack), expected);
        }
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../input.txt");
        assert_eq!(part1(input), 392367);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../input.txt");
        assert_eq!(part2(input), 2192104158);
    }
}
