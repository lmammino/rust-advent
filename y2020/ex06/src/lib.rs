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

pub fn part2(input: &str) -> usize {
    let mut answers  = 0;
    // this is true only for the first line of a block of answers
    let mut is_first = true;

    let mut question_answers_set = HashSet::new();
    for line in input.lines() {
        if line.is_empty() {
            answers += question_answers_set.len();
            question_answers_set.clear();
            is_first = true;
        } else {
            if is_first {
                question_answers_set = line.chars().collect();
                // for c in line.chars() {
                //     question_answers_set.insert(c);
                // }
                is_first = false;
            } else {
                // let mut current_answers_set = HashSet::new();
                // for c in line.chars() {
                //     current_answers_set.insert(c);
                // }
                question_answers_set = question_answers_set.intersection(&line.chars().collect()).map(|c | *c).collect();
            }
        }
    }

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

    #[test]
    fn part_2() {
        let input = include_str!("../input.txt");
        assert_eq!(part2(input), 3316)
    }
    
}
