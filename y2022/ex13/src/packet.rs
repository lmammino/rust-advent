use std::{cmp::Ordering, str::FromStr};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::line_ending,
    multi::{many1, separated_list0},
    sequence::{delimited, separated_pair},
    IResult,
};

#[derive(Debug, Clone, Eq, PartialEq)]
pub(crate) enum Packet {
    List(Vec<Packet>),
    Val(u64),
}

impl FromStr for Packet {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_packet(s).map(|(_, p)| p).map_err(|e| e.to_string())
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Packet::Val(a), Packet::Val(b)) => a.cmp(b),
            (Packet::List(a), Packet::List(b)) => {
                let mut i = 0;
                loop {
                    let left = a.get(i);
                    let right = b.get(i);

                    match (left, right) {
                        (None, None) => return Ordering::Equal,
                        (None, Some(_)) => return Ordering::Less,
                        (Some(_), None) => return Ordering::Greater,
                        (Some(left), Some(right)) => {
                            let cmp = left.cmp(right);
                            if cmp == Ordering::Equal {
                                i += 1;
                                continue;
                            } else {
                                return cmp;
                            }
                        }
                    }
                }
            }
            (Packet::Val(a), Packet::List(_)) => {
                let left = Packet::List(vec![Packet::Val(*a)]);
                left.cmp(other)
            }
            (Packet::List(_), Packet::Val(b)) => {
                let right = Packet::List(vec![Packet::Val(*b)]);
                self.cmp(&right)
            }
        }
    }
}

fn parse_packet_value(input: &str) -> IResult<&str, Packet> {
    let (input, num) = nom::character::complete::u64(input)?;
    Ok((input, Packet::Val(num)))
}

fn parse_packet_list(input: &str) -> IResult<&str, Packet> {
    let (input, list) =
        delimited(tag("["), separated_list0(tag(","), parse_packet), tag("]"))(input)?;
    Ok((input, Packet::List(list)))
}

fn parse_packet(input: &str) -> IResult<&str, Packet> {
    let (input, packet) = alt((parse_packet_value, parse_packet_list))(input)?;
    Ok((input, packet))
}

fn parse_pair(input: &str) -> IResult<&str, (Packet, Packet)> {
    let (input, (left, right)) = separated_pair(parse_packet, line_ending, parse_packet)(input)?;
    Ok((input, (left, right)))
}

pub(crate) fn parse_input_in_pairs(input: &str) -> impl Iterator<Item = (Packet, Packet)> + '_ {
    input.split("\n\n").map(|s| parse_pair(s).unwrap().1)
}

pub(crate) fn parse_input_as_list(input: &str) -> IResult<&str, Vec<Packet>> {
    separated_list0(many1(line_ending), parse_packet)(input)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_input_in_pairs() {
        use Packet::*;

        let input = "[[[[4,7,7],0,4,[6,3,3,7,10],2],2,[[3]]],[]]
[[[[1,4,8],[3,3,1,4]]]]

[[7,6],[],[[5],0,10,[7,9,[7],0]]]
[[[[6,9,0]],0]]";

        let expected = [
            (
                List(vec![
                    List(vec![
                        List(vec![
                            List(vec![Val(4), Val(7), Val(7)]),
                            Val(0),
                            Val(4),
                            List(vec![Val(6), Val(3), Val(3), Val(7), Val(10)]),
                            Val(2),
                        ]),
                        Val(2),
                        List(vec![List(vec![Val(3)])]),
                    ]),
                    List(vec![]),
                ]),
                List(vec![List(vec![List(vec![
                    List(vec![Val(1), Val(4), Val(8)]),
                    List(vec![Val(3), Val(3), Val(1), Val(4)]),
                ])])]),
            ),
            (
                List(vec![
                    List(vec![Val(7), Val(6)]),
                    List(vec![]),
                    List(vec![
                        List(vec![Val(5)]),
                        Val(0),
                        Val(10),
                        List(vec![Val(7), Val(9), List(vec![Val(7)]), Val(0)]),
                    ]),
                ]),
                List(vec![List(vec![
                    List(vec![List(vec![Val(6), Val(9), Val(0)])]),
                    Val(0),
                ])]),
            ),
        ];

        let packets: Vec<(Packet, Packet)> = parse_input_in_pairs(input).collect();
        assert_eq!(packets.len(), 2);
        assert_eq!(packets, expected);
    }

    #[test]
    fn test_parse_input_as_list() {
        use Packet::*;

        let input = "[[[[4,7,7],0,4,[6,3,3,7,10],2],2,[[3]]],[]]
[[[[1,4,8],[3,3,1,4]]]]

[[7,6],[],[[5],0,10,[7,9,[7],0]]]
[[[[6,9,0]],0]]";

        let expected = [
            List(vec![
                List(vec![
                    List(vec![
                        List(vec![Val(4), Val(7), Val(7)]),
                        Val(0),
                        Val(4),
                        List(vec![Val(6), Val(3), Val(3), Val(7), Val(10)]),
                        Val(2),
                    ]),
                    Val(2),
                    List(vec![List(vec![Val(3)])]),
                ]),
                List(vec![]),
            ]),
            List(vec![List(vec![List(vec![
                List(vec![Val(1), Val(4), Val(8)]),
                List(vec![Val(3), Val(3), Val(1), Val(4)]),
            ])])]),
            List(vec![
                List(vec![Val(7), Val(6)]),
                List(vec![]),
                List(vec![
                    List(vec![Val(5)]),
                    Val(0),
                    Val(10),
                    List(vec![Val(7), Val(9), List(vec![Val(7)]), Val(0)]),
                ]),
            ]),
            List(vec![List(vec![
                List(vec![List(vec![Val(6), Val(9), Val(0)])]),
                Val(0),
            ])]),
        ];

        let (_, packets) = parse_input_as_list(input).unwrap();
        assert_eq!(packets.len(), 4);
        assert_eq!(packets, expected);
    }

    #[test]
    fn test_sorting() {
        let cases = [
            ("[1,1,3,1,1]", "[1,1,5,1,1]", Ordering::Less),
            ("[[1],[2,3,4]]", "[[1],4]", Ordering::Less),
            ("[9]", "[[8,7,6]]", Ordering::Greater),
            ("[[4,4],4,4]", "[[4,4],4,4,4]", Ordering::Less),
            ("[7,7,7,7]", "[7,7,7]", Ordering::Greater),
            ("[]", "[3]", Ordering::Less),
            ("[[[]]]", "[[]]", Ordering::Greater),
            (
                "[1,[2,[3,[4,[5,6,7]]]],8,9]",
                "[1,[2,[3,[4,[5,6,0]]]],8,9]",
                Ordering::Greater,
            ),
            (
                "[1,[2,[3,[4,[5,6,7]]]],8,9]",
                "[1,[2,[3,[4,[5,6,7]]]],8,9]",
                Ordering::Equal,
            ),
        ];

        for (left, right, expected) in cases.iter() {
            let left = parse_packet(left).unwrap().1;
            let right = parse_packet(right).unwrap().1;
            assert_eq!(left.cmp(&right), *expected, "{:?} vs {:?}", left, right);
        }
    }
}
