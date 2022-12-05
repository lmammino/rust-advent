use nom::{
    branch::alt,
    bytes::complete::{is_not, tag},
    character::complete::digit1,
    combinator::map_res,
    sequence::separated_pair,
    IResult,
};

#[derive(Debug, PartialEq)]
pub(crate) enum Cmd<'a> {
    Cd(&'a str),
    Ls,
}

#[derive(Debug, PartialEq)]
pub(crate) enum Out<'a> {
    Dir(&'a str),
    File(usize, &'a str),
}

#[derive(Debug, PartialEq)]
pub(crate) enum Line<'a> {
    CmdLine(Cmd<'a>),
    OutLine(Out<'a>),
}

pub(crate) fn parse_cmd_cd(input: &str) -> IResult<&str, Line> {
    let (input, _) = tag("$ cd ")(input)?;
    let (input, name) = is_not("\n\r")(input)?;

    Ok((input, Line::CmdLine(Cmd::Cd(name.trim()))))
}

pub(crate) fn parse_cmd_ls(input: &str) -> IResult<&str, Line> {
    let (input, _) = tag("$ ls")(input)?;

    Ok((input, Line::CmdLine(Cmd::Ls)))
}

pub(crate) fn parse_output_dir(input: &str) -> IResult<&str, Line> {
    let (input, _) = tag("dir ")(input)?;
    let (input, name) = is_not("\n\r")(input)?;

    Ok((input, Line::OutLine(Out::Dir(name.trim()))))
}

pub(crate) fn parse_output_file(input: &str) -> IResult<&str, Line> {
    let mut parser = separated_pair(
        map_res(digit1, |s: &str| s.parse::<usize>()),
        tag(" "),
        is_not("\n\r"),
    );
    let (input, (size, name)) = parser(input)?;

    Ok((input, Line::OutLine(Out::File(size, name.trim()))))
}

pub(crate) fn parse_line(input: &str) -> IResult<&str, Line> {
    alt((
        parse_cmd_cd,
        parse_cmd_ls,
        parse_output_dir,
        parse_output_file,
    ))(input)
}

pub(crate) fn parse_input(input: &str) -> impl Iterator<Item = Line> {
    input.lines().map(|l| parse_line(l).unwrap().1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_cmd_cd() {
        let input = "$ cd /";
        assert_eq!(parse_cmd_cd(input), Ok(("", Line::CmdLine(Cmd::Cd("/")))));

        let input = "$ cd ..";
        assert_eq!(parse_cmd_cd(input), Ok(("", Line::CmdLine(Cmd::Cd("..")))));

        let input = "$ cd lwhbw";
        assert_eq!(
            parse_cmd_cd(input),
            Ok(("", Line::CmdLine(Cmd::Cd("lwhbw"))))
        );
    }

    #[test]
    fn test_parse_cmd_ls() {
        let input = "$ ls";
        assert_eq!(parse_cmd_ls(input), Ok(("", Line::CmdLine(Cmd::Ls))));
    }

    #[test]
    fn test_parse_output_dir() {
        let input = "dir sqhw";
        assert_eq!(
            parse_output_dir(input),
            Ok(("", Line::OutLine(Out::Dir("sqhw"))))
        );
    }

    #[test]
    fn test_parse_output_file() {
        let input = "155241 qvnbd.dqs";
        assert_eq!(
            parse_output_file(input),
            Ok(("", Line::OutLine(Out::File(155241, "qvnbd.dqs"))))
        );

        let input = "6655 tndtmwfv";
        assert_eq!(
            parse_output_file(input),
            Ok(("", Line::OutLine(Out::File(6655, "tndtmwfv"))))
        );
    }

    #[test]
    fn test_parse_input() {
        let input = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";
        let lines = parse_input(input);
        assert_eq!(
            lines.collect::<Vec<_>>(),
            vec![
                Line::CmdLine(Cmd::Cd("/")),
                Line::CmdLine(Cmd::Ls),
                Line::OutLine(Out::Dir("a")),
                Line::OutLine(Out::File(14848514, "b.txt")),
                Line::OutLine(Out::File(8504156, "c.dat")),
                Line::OutLine(Out::Dir("d")),
                Line::CmdLine(Cmd::Cd("a")),
                Line::CmdLine(Cmd::Ls),
                Line::OutLine(Out::Dir("e")),
                Line::OutLine(Out::File(29116, "f")),
                Line::OutLine(Out::File(2557, "g")),
                Line::OutLine(Out::File(62596, "h.lst")),
                Line::CmdLine(Cmd::Cd("e")),
                Line::CmdLine(Cmd::Ls),
                Line::OutLine(Out::File(584, "i")),
                Line::CmdLine(Cmd::Cd("..")),
                Line::CmdLine(Cmd::Cd("..")),
                Line::CmdLine(Cmd::Cd("d")),
                Line::CmdLine(Cmd::Ls),
                Line::OutLine(Out::File(4060174, "j")),
                Line::OutLine(Out::File(8033020, "d.log")),
                Line::OutLine(Out::File(5626152, "d.ext")),
                Line::OutLine(Out::File(7214296, "k")),
            ]
        );
    }
}
