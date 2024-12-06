// Implementing this bespoke matcher because i am on a plane
// without connectivity and I can't install the regex crate -.-

use State::*;

#[derive(Debug, Clone)]
enum State {
    LookForM,
    LookForU,
    LookForL,
    LookForOpenParen,
    LookForFirstNumber,
    LookForSecondNumber,
}

impl Default for State {
    fn default() -> Self {
        Self::LookForM
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Match {
    Mul(u64, u64),
    Do,
    Dont,
}

#[derive(Debug, Clone, Default)]
pub struct Matcher {
    state: State,
    left: String,
    right: String,
}

impl Matcher {
    fn reset(&mut self) {
        self.state = LookForM;
        self.left = "".to_string();
        self.right = "".to_string();
    }

    fn eat<'a>(&mut self, s: &'a str) -> (Option<Match>, &'a str) {
        if s.starts_with("do()") {
            self.reset();
            return (Some(Match::Do), s.strip_prefix("do()").unwrap());
        } else if s.starts_with("don't()") {
            self.reset();
            return (Some(Match::Dont), s.strip_prefix("don't()").unwrap());
        }

        match (&self.state, s.chars().next().unwrap()) {
            (LookForM, 'm') => self.state = LookForU,
            (LookForU, 'u') => self.state = LookForL,
            (LookForL, 'l') => self.state = LookForOpenParen,
            (LookForOpenParen, '(') => self.state = LookForFirstNumber,
            (LookForFirstNumber, c) => {
                if c.is_ascii_digit() {
                    self.left.push(c);
                } else if c == ',' && !self.left.is_empty() {
                    self.state = LookForSecondNumber;
                } else {
                    self.reset();
                }
            }
            (LookForSecondNumber, c) => {
                if c.is_ascii_digit() {
                    self.right.push(c);
                } else if c == ')' && !self.right.is_empty() {
                    let l: u64 = self.left.parse().unwrap();
                    let r: u64 = self.right.parse().unwrap();
                    self.reset();
                    return (Some(Match::Mul(l, r)), &s[1..]);
                } else {
                    self.reset();
                }
            }
            _ => self.reset(),
        }

        (None, &s[1..])
    }
}

#[derive(Debug)]
pub struct MatcherIter<'a> {
    s: &'a str,
    matcher: Matcher,
}

impl<'a> Iterator for MatcherIter<'a> {
    type Item = Match;

    fn next(&mut self) -> Option<Self::Item> {
        while !self.s.is_empty() {
            let (m, s) = self.matcher.eat(self.s);
            self.s = s;

            if let Some(m) = m {
                return Some(m);
            }
        }

        None
    }
}

impl<'a> MatcherIter<'a> {
    pub fn new(s: &'a str) -> Self {
        Self {
            s,
            matcher: Default::default(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_match() {
        let s = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let matches: Vec<Match> = MatcherIter::new(s).collect();
        assert_eq!(
            matches,
            vec![
                Match::Mul(2, 4),
                Match::Mul(5, 5),
                Match::Mul(11, 8),
                Match::Mul(8, 5)
            ]
        )
    }
}
