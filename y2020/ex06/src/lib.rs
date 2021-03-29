use std::collections::HashSet;

pub fn part1(input: &str) -> usize {
    let mut answers  = 0;

    let mut question_answers_set = HashSet::new();
    for line in input.lines() {
        if line.is_empty() {
            answers += question_answers_set.len();
            question_answers_set.clear();
        } else {
            for c in line.chars() {
                question_answers_set.insert(c);
            }
        }
    }
    // This is for the last answers (in case there is no empty line after)
    answers += question_answers_set.len();
    
    answers
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let input = include_str!("../input.txt");
        assert_eq!(part1(input), 6726)
    }

    // part 2 3316
}
