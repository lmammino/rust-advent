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

#[derive(Debug, Clone)]
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

    // TODO: unify with previous method
    fn play_with_roll(&mut self, roll: usize) {
        let new_pos = (self.pos + roll) % 10;
        self.score += new_pos + 1;
        self.pos = new_pos;
    }

    fn has_won_part1(&self) -> bool {
        self.score >= 1000
    }

    fn has_won_part2(&self) -> bool {
        self.score >= 21
    }
}

#[derive(Debug, Clone)]
struct Game {
    players: [Player; 2],
    current_player: usize,
}

impl Game {
    fn new(player1: Player, player2: Player) -> Self {
        Game {
            players: [player1, player2],
            current_player: 0,
        }
    }

    fn play_turn(&mut self, die: usize) {
        let player = &mut self.players[self.current_player];
        player.play_with_roll(die);
        self.current_player = (self.current_player + 1) % self.players.len();
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
        if current_player.has_won_part1() {
            return die.num_rolls * other_player.score;
        }
        std::mem::swap(&mut current_player, &mut other_player);
    }
}

pub fn part2(input: &str) -> usize {
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

    let (player1, player2) = (Player::new(player1_pos - 1), Player::new(player2_pos - 1));

    let game = Game::new(player1, player2);

    let mut victories = [0_usize, 0_usize];
    let mut games: Vec<(Game, usize)> = vec![(game, 1)];

    // At every turn, every time we roll a die we split the universe in 3.
    // Every universe will represent a different outcome of rolling the die (1, 2, or 3).
    // Because we have to roll the die 3 times, we end up with 27 new universes after every
    // turn.
    // The total of a turn is the sum of the 3 dice. This means that the total can vary between
    // 3 (worst score) to 9 (best score).
    // The following table encodes all the possible scores (3..9) and how many chances do they have
    // to happen (3 & 9 only once, 6 will happen 7 times per turn).
    // By doing this we can reduce the number of simulation to do, because we don't need
    // to run all the 27 simulations (per turn) but we can just run the 7 different cases.
    // We will then have to multiple the outcome of every case to how many times that case will repeat.
    let universes = [
        (3_usize, 1_usize),
        (4, 3),
        (5, 6),
        (6, 7),
        (7, 6),
        (8, 3),
        (9, 1),
    ];

    // TODO: impl logic
    while !games.is_empty() {
        let (current_game, weight) = games.pop().unwrap();

        // if the game is over accumulate the victories for the winning player
        let mut is_over = false;
        for (i, victory) in victories.iter_mut().enumerate() {
            if current_game.players[i].has_won_part2() {
                *victory += weight;
                is_over = true;
                break;
            }
        }
        if is_over {
            continue;
        }

        // if the current game is not over, keep splitting it
        for (roll, new_weight) in universes.iter() {
            let mut new_game = current_game.clone();
            new_game.play_turn(*roll);

            games.push((new_game, weight * new_weight));
        }
    }

    *victories.iter().max().unwrap()
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

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 430229563871565);
    }
}
