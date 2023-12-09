use nom::{
    character::complete::{i64, space1},
    multi::separated_list1,
    IResult,
};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct History(Vec<Vec<i64>>);

impl History {
    fn new(fist_line: Vec<i64>) -> Self {
        Self(vec![fist_line])
    }

    pub fn expand(&mut self) {
        let mut last_line = self.0.last().unwrap();
        while !last_line.iter().all(|&n| n == 0) {
            let diffs = last_line
                .windows(2)
                .map(|w| w[1] - w[0])
                .collect::<Vec<i64>>();
            self.0.push(diffs);
            last_line = self.0.last().unwrap();
        }
    }

    pub fn extrapolate_right(&mut self) -> i64 {
        let mut last_prediction = 0;
        // adds a 0 to the last line
        self.0.last_mut().unwrap().push(0);

        // starts to predict bottom-up
        for line_idx in (1..self.0.len()).rev() {
            let line_last_value = *self.0.get(line_idx).unwrap().last().unwrap();
            let prev_line = self.0.get_mut(line_idx - 1).unwrap();
            let prev_line_last_value = prev_line.last().unwrap();

            let new_value = line_last_value + prev_line_last_value;
            prev_line.push(new_value);
            last_prediction = new_value;
        }

        last_prediction
    }

    pub fn extrapolate_left(&mut self) -> i64 {
        let mut last_prediction = 0;
        // adds a 0 at the beginning of the last line
        self.0.last_mut().unwrap().insert(0, 0);

        // starts to predict bottom-up
        for line_idx in (1..self.0.len()).rev() {
            let line_first_value = *self.0.get(line_idx).unwrap().first().unwrap();
            let prev_line = self.0.get_mut(line_idx - 1).unwrap();
            let prev_line_first_value = prev_line.first().unwrap();

            let new_value = prev_line_first_value - line_first_value;
            prev_line.insert(0, new_value);
            last_prediction = new_value;
        }

        last_prediction
    }
}

pub fn parse_history(input: &str) -> IResult<&str, History> {
    let (input, first_line) = separated_list1(space1, i64)(input)?;
    Ok((input, History::new(first_line)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_extrapolate() {
        let (_, mut history) = parse_history("0 3 6 9 12 15").unwrap();
        assert_eq!(history, History(vec![vec![0, 3, 6, 9, 12, 15]]));

        history.expand();
        assert_eq!(
            history,
            History(vec![
                vec![0, 3, 6, 9, 12, 15],
                vec![3, 3, 3, 3, 3],
                vec![0, 0, 0, 0],
            ])
        );

        let predictions = history.extrapolate_right();
        assert_eq!(
            history,
            History(vec![
                vec![0, 3, 6, 9, 12, 15, 18],
                vec![3, 3, 3, 3, 3, 3],
                vec![0, 0, 0, 0, 0],
            ])
        );
        assert_eq!(predictions, 18)
    }

    #[test]
    fn test_parse_extrapolate2() {
        let (_, mut history) = parse_history("1 3 6 10 15 21").unwrap();
        assert_eq!(history, History(vec![vec![1, 3, 6, 10, 15, 21]]));

        history.expand();
        assert_eq!(
            history,
            History(vec![
                vec![1, 3, 6, 10, 15, 21],
                vec![2, 3, 4, 5, 6],
                vec![1, 1, 1, 1],
                vec![0, 0, 0],
            ])
        );

        let predictions = history.extrapolate_right();
        assert_eq!(
            history,
            History(vec![
                vec![1, 3, 6, 10, 15, 21, 28],
                vec![2, 3, 4, 5, 6, 7],
                vec![1, 1, 1, 1, 1],
                vec![0, 0, 0, 0],
            ])
        );
        assert_eq!(predictions, 28)
    }

    #[test]
    fn test_parse_extrapolate3() {
        let (_, mut history) = parse_history("10 13 16 21 30 45").unwrap();
        assert_eq!(history, History(vec![vec![10, 13, 16, 21, 30, 45]]));

        history.expand();
        assert_eq!(
            history,
            History(vec![
                vec![10, 13, 16, 21, 30, 45],
                vec![3, 3, 5, 9, 15],
                vec![0, 2, 4, 6],
                vec![2, 2, 2],
                vec![0, 0],
            ])
        );

        let mut history_left = history.clone();

        let predictions = history.extrapolate_right();
        assert_eq!(
            history,
            History(vec![
                vec![10, 13, 16, 21, 30, 45, 68],
                vec![3, 3, 5, 9, 15, 23],
                vec![0, 2, 4, 6, 8],
                vec![2, 2, 2, 2],
                vec![0, 0, 0],
            ])
        );
        assert_eq!(predictions, 68);

        let predictions = history_left.extrapolate_left();
        assert_eq!(
            history_left,
            History(vec![
                vec![5, 10, 13, 16, 21, 30, 45],
                vec![5, 3, 3, 5, 9, 15],
                vec![-2, 0, 2, 4, 6],
                vec![2, 2, 2, 2],
                vec![0, 0, 0],
            ])
        );
        assert_eq!(predictions, 5);
    }
}
