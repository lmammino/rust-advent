#[derive(Debug, Clone, PartialEq)]
enum Pos {
    L,
    R,
}

type SNumExpr = Vec<SNum>;

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

fn parse(line: &str) -> SNumExpr {
    let mut expr: SNumExpr = Vec::new();
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

fn sum(expr1: &SNumExpr, expr2: &SNumExpr) -> SNumExpr {
    let mut result: SNumExpr = vec![];

    // left treee
    for num in expr1 {
        let mut new_num = num.clone();
        new_num.position.insert(0, Pos::L);
        result.push(new_num);
    }

    // right tree
    for num in expr2 {
        let mut new_num = num.clone();
        new_num.position.insert(0, Pos::R);
        result.push(new_num);
    }

    while reduce(&mut result) {}

    result
}

pub fn part1(input: &str) -> u32 {
    let mut expressions = input.lines().map(parse);
    let first_expr = expressions.next().unwrap();
    let resulting_sum = expressions.fold(first_expr, |acc, curr| sum(&acc, &curr));

    magnitude(&resulting_sum)
}

pub fn part2(input: &str) -> u32 {
    let expressions: Vec<SNumExpr> = input.lines().map(parse).collect();
    let mut max = 0;

    for i in 0..expressions.len() {
        for j in 0..expressions.len() {
            if i != j {
                let expr1 = expressions.get(i).unwrap();
                let expr2 = expressions.get(j).unwrap();
                let val = magnitude(&sum(expr1, expr2));
                if val > max {
                    max = val
                }
            }
        }
    }

    max
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

    #[test]
    fn test_sum() {
        let cases = vec![
            (
                "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]",
                "[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]",
                "[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]",
            ),
            (
                "[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]",
                "[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]",
                "[[[[6,7],[6,7]],[[7,7],[0,7]]],[[[8,7],[7,7]],[[8,8],[8,0]]]]",
            ),
            (
                "[[[[6,7],[6,7]],[[7,7],[0,7]]],[[[8,7],[7,7]],[[8,8],[8,0]]]]",
                "[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]",
                "[[[[7,0],[7,7]],[[7,7],[7,8]]],[[[7,7],[8,8]],[[7,7],[8,7]]]]",
            ),
            (
                "[[[[7,0],[7,7]],[[7,7],[7,8]]],[[[7,7],[8,8]],[[7,7],[8,7]]]]",
                "[7,[5,[[3,8],[1,4]]]]",
                "[[[[7,7],[7,8]],[[9,5],[8,7]]],[[[6,8],[0,8]],[[9,9],[9,0]]]]",
            ),
        ];

        for (expr1, expr2, expected) in cases {
            assert_eq!(sum(&parse(expr1), &parse(expr2)), parse(expected));
        }
    }
}
