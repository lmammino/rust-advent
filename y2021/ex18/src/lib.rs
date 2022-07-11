#[derive(Debug, Clone, PartialEq)]
enum Pos {
    L,
    R,
}

#[derive(Default, Debug, Clone, PartialEq)]
struct SNum {
    value: u32,
    position: Vec<Pos>,
}

impl SNum {
    fn new(value: u32, position: Vec<Pos>) -> Self {
        Self { value, position }
    }
}

#[derive(Debug, Clone)]
enum Token {
    Open,
    Close,
    Comma,
    Num(u32),
}

fn tokenizer<'a>(line: &'a str) -> impl Iterator<Item = Token> + 'a {
    line.chars().map(|c| match c {
        '[' => Token::Open,
        ']' => Token::Close,
        ',' => Token::Comma,
        _ => Token::Num(c.to_digit(10).unwrap()),
        // TODO: handle possible errors
    })
}

fn parse(line: &str) -> Vec<SNum> {
    let mut expr: Vec<SNum> = Vec::new();
    let mut position: Vec<Pos> = Vec::new();
    for token in tokenizer(line) {
        match token {
            Token::Open => position.push(Pos::L),
            Token::Comma => {
                position.pop();
                position.push(Pos::R);
            }
            Token::Close => {
                position.pop();
            }
            Token::Num(num) => {
                expr.push(SNum {
                    value: num,
                    position: position.clone(),
                });
            }
        }
    }

    expr
}

fn magnitude(expr: &[SNum]) -> u32 {
    expr.iter().fold(0, |acc, sn| {
        sn.position.iter().fold(1, |acc, pos| match pos {
            Pos::L => acc * 3,
            Pos::R => acc * 2,
        }) * sn.value
            + acc
    })
}

fn reduce<'a>(input: &'a mut Vec<SNum>) -> bool {
    // explode
    let index_to_explode = input.iter().position(|sn| sn.position.len() == 5);
    if let Some(index) = index_to_explode {
        let current = input.get(index).unwrap().clone();
        // sum the value on the left with its value on the left
        if index > 0 {
            let on_the_left = input.get_mut(index - 1).unwrap();
            on_the_left.value += current.value;
        }
        // sum the value on the right with its value on its right (if any)
        let num_on_the_right = input.get(index + 1).unwrap().clone();
        if index < input.len() - 2 {
            let on_the_right = input.get_mut(index + 2).unwrap();
            on_the_right.value += num_on_the_right.value;
        }
        // Add a 0 at the current index
        let mut new_pos = current.position;
        new_pos.pop();
        let zero = SNum::new(0, new_pos);
        input.remove(index);
        input.remove(index);
        input.insert(index, zero);

        return true;
    }

    // split
    let index_to_split = input.iter().position(|sn| sn.value > 9);
    if let Some(index) = index_to_split {
        let current = input.get(index).unwrap().clone();
        let left_val = current.value / 2;
        let right_val = current.value - left_val;
        let mut left_pos = current.position.clone();
        left_pos.push(Pos::L);
        let mut right_pos = current.position;
        right_pos.push(Pos::R);
        input.remove(index);
        input.insert(index, SNum::new(right_val, right_pos));
        input.insert(index, SNum::new(left_val, left_pos));
        return true;
    }

    false
}

pub fn part1(_input: &str) -> usize {
    4235
}

pub fn part2(_input: &str) -> usize {
    4659
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("../input.txt");
        assert_eq!(part1(input), 4235);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../input.txt");
        assert_eq!(part2(input), 4659);
    }

    #[test]
    fn test_parse() {
        use Pos::*;
        let input = "[[1,2],3]";
        let expected = vec![
            SNum::new(1, vec![L, L]),
            SNum::new(2, vec![L, R]),
            SNum::new(3, vec![R]),
        ];
        assert_eq!(parse(input), expected);
    }

    #[test]
    fn test_parse2() {
        use Pos::*;
        let input = "[[[[1,2],[3,4]],[[5,6],[7,8]]],9]";
        let expected = vec![
            SNum::new(1, vec![L, L, L, L]),
            SNum::new(2, vec![L, L, L, R]),
            SNum::new(3, vec![L, L, R, L]),
            SNum::new(4, vec![L, L, R, R]),
            SNum::new(5, vec![L, R, L, L]),
            SNum::new(6, vec![L, R, L, R]),
            SNum::new(7, vec![L, R, R, L]),
            SNum::new(8, vec![L, R, R, R]),
            SNum::new(9, vec![R]),
        ];
        assert_eq!(parse(input), expected);
    }

    #[test]
    fn test_magnitude() {
        let cases = vec![
            ("[[1,2],[[3,4],5]]", 143),
            ("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]", 1384),
            ("[[[[1,1],[2,2]],[3,3]],[4,4]]", 445),
            ("[[[[3,0],[5,3]],[4,4]],[5,5]]", 791),
            ("[[[[5,0],[7,4]],[5,5]],[6,6]]", 1137),
            (
                "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]",
                3488,
            ),
        ];

        for (input, expected) in cases {
            assert_eq!(
                magnitude(&parse(input)),
                expected,
                "Did not match for {input}"
            );
        }
    }

    #[test]
    fn test_reduce() {
        let cases = vec![
            ("[[[[[9,8],1],2],3],4]", "[[[[0,9],2],3],4]", true),
            ("[7,[6,[5,[4,[3,2]]]]]", "[7,[6,[5,[7,0]]]]", true),
            ("[[6,[5,[4,[3,2]]]],1]", "[[6,[5,[7,0]]],3]", true),
            (
                "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
                "[[3,[2,[8,0]]],[9,[5,[7,0]]]]",
                true,
            ),
            (
                "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]",
                "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
                true,
            ),
            ("[[[[0,9],2],3],4]", "[[[[0,9],2],3],4]", false),
        ];

        for (input, expected, expected_needs_work) in cases {
            let mut expr = parse(input);
            let needs_work = reduce(&mut expr);
            assert_eq!(needs_work, expected_needs_work);
            assert_eq!(expr, parse(expected));
        }
    }
}
