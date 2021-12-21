#[derive(Debug)]
struct DeterministicDie {
    current: usize,
    num_rolls: usize,
}

impl DeterministicDie {
    fn new() -> Self {
        DeterministicDie {
            current: 0,
            num_rolls: 0,
        }
    }

    fn roll(&mut self) -> usize {
        if self.current == 100 {
            self.current = 1;
        } else {
            self.current += 1;
        }

        self.num_rolls += 1;
        self.current
    }

    fn roll3(&mut self) -> (usize, usize, usize) {
        (self.roll(), self.roll(), self.roll())
    }
}

#[derive(Debug)]
struct Player {
    pos: usize,
    score: usize,
}

impl Player {
    fn new(pos: usize) -> Self {
        Player { pos, score: 0 }
    }

    fn play(&mut self, die: &mut DeterministicDie) {
        let rolls = die.roll3();
        let tot = rolls.0 + rolls.1 + rolls.2;
        let new_pos = (self.pos + tot) % 10;
        self.score += new_pos + 1;
        self.pos = new_pos;
    }

    fn has_won(&self) -> bool {
        self.score >= 1000
    }
}

pub fn part1(input: &str) -> usize {
    let mut l = input.lines();
    let player1_pos: usize = l
        .next()
        .unwrap()
        .split(": ")
        .nth(1)
        .unwrap()
        .parse()
        .unwrap();
    let player2_pos: usize = l
        .next()
        .unwrap()
        .split(": ")
        .nth(1)
        .unwrap()
        .parse()
        .unwrap();

    let (mut player1, mut player2) = (Player::new(player1_pos - 1), Player::new(player2_pos - 1));
    let mut current_player = &mut player1;
    let mut other_player = &mut player2;
    let mut die = DeterministicDie::new();

    loop {
        current_player.play(&mut die);
        if current_player.has_won() {
            return die.num_rolls * other_player.score;
        }
        std::mem::swap(&mut current_player, &mut other_player);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("../input.txt");

    #[test]
    fn test_example() {
        let input = "Player 1 starting position: 4
Player 2 starting position: 8";

        assert_eq!(part1(input), 739785);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 989352);
    }
}
