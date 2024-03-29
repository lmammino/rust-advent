use nom::{
    bytes::complete::tag,
    character::complete::{newline, space1, u64},
    combinator::complete,
    multi::separated_list1,
    IResult,
};

#[derive(Debug, PartialEq, Eq)]
struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    fn num_winning_configurations(&self) -> u64 {
        // if we analyse the first example with time = 7 and distance = 9
        // we can have the following configurations:
        //
        // speed = 1 -> 1 + 1 + 1 + 1 + 1 + 1 =  6
        // speed = 2 -> 2 + 2 + 2 + 2 + 2     = 10
        // speed = 3 -> 3 + 3 + 3 + 3         = 12
        // speed = 4 -> 4 + 4 + 4             = 12
        // speed = 5 -> 5 + 5                 = 10
        // speed = 6 -> 6                     =  6
        // We can use the formula
        //      MAXdist = speed * (time - speed)
        // for any possible speed to calculate the maximum distance that can be travelled
        //
        // This means that we can calculate the number of winning configurations by solving
        // the following inequation where speed is the unknown variable:
        //     speed * (time - speed) > distance
        // which is equivalent to
        //     speed^2 - time * speed + distance < 0
        //
        // the roots of this equation are
        //     s1 = (time - sqrt(time^2 - 4 * distance)) / 2
        //     s2 = (time + sqrt(time^2 - 4 * distance)) / 2
        //
        // At this point we only need to count the discrete points between s1 and s2
        let t = self.time as i64;
        let d = self.distance as i64;

        let k = ((t * t - 4 * d) as f64).sqrt();
        let s1 = (t as f64 - k) / 2.0;
        let s2 = (t as f64 + k) / 2.0;

        // make sure to adjust for correctly accounting for continuous to discrete points
        // e.g. if we have to count all the points in (10..20), we have 9 discrete points
        // but if we have to count all the points in (10.1..20.1), we have 10 discrete points (because 20 is now included in the solution space)
        let m = if s1.fract() == 0.0 && s2.fract() == 0.0 {
            -1
        } else {
            0
        };

        ((s2.floor() - s1.floor()) + m as f64) as u64
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Boat {
    speed: u64,
    distance_travelled: u64,
    time_passed: u64,
}

impl Boat {
    fn new(speed: u64) -> Self {
        Self {
            speed,
            distance_travelled: 0,
            time_passed: 0,
        }
    }

    fn tick(&mut self) {
        self.distance_travelled += self.speed;
        self.time_passed += 1;
    }

    fn race(mut self, time_limit: u64) -> u64 {
        self.time_passed = self.speed; // time spent pressing the button
        while self.time_passed < time_limit {
            // race
            self.tick();
        }
        self.distance_travelled
    }
}

fn parse_input(input: &str) -> IResult<&str, Vec<Race>> {
    // Time:      7  15   30
    // Distance:  9  40  200
    let (input, _) = tag("Time:")(input)?;
    let (input, _) = space1(input)?;
    let (input, times) = separated_list1(space1, u64)(input)?;
    let (input, _) = newline(input)?;
    let (input, _) = tag("Distance:")(input)?;
    let (input, _) = space1(input)?;
    let (input, distances) = complete(separated_list1(space1, u64))(input)?;
    let races: Vec<Race> = times
        .iter()
        .zip(distances.iter())
        .map(|(time, distance)| Race {
            time: *time,
            distance: *distance,
        })
        .collect();

    Ok((input, races))
}

pub fn part1(input: &str) -> u64 {
    let (_, races) = parse_input(input).unwrap();
    races
        .iter()
        .map(|race| {
            // // Brute force method:
            // let mut winning_count = 0;
            // for i in 1..race.time {
            //     let boat = Boat::new(i);
            //     let distance_travelled = boat.race(race.time);
            //     if distance_travelled > race.distance {
            //         winning_count += 1;
            //     }
            // }
            // winning_count
            race.num_winning_configurations()
        })
        .product()
}

pub fn part2(input: &str) -> u64 {
    let (_, races) = parse_input(input).unwrap();
    let (time, distance) = races.iter().fold(
        (String::new(), String::new()),
        |(time, distance), curr_race| {
            let mut time = time.clone();
            time.push_str(curr_race.time.to_string().as_str());
            let mut distance = distance.clone();
            distance.push_str(curr_race.distance.to_string().as_str());
            (time, distance)
        },
    );
    let time: u64 = time.parse().unwrap();
    let distance: u64 = distance.parse().unwrap();
    let race = Race { time, distance };

    // // Brute force method:
    // let mut winning_count = 0;
    // for i in 1..race.time {
    //     let boat = Boat::new(i);
    //     let distance_travelled = boat.race(race.time);
    //     if distance_travelled > race.distance {
    //         winning_count += 1;
    //     }
    // }
    // winning_count
    race.num_winning_configurations()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("../input.txt");
    const EXAMPLE_INPUT: &str = "Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn test_parse() {
        let (_, races) = parse_input(EXAMPLE_INPUT).unwrap();
        assert_eq!(
            races,
            vec![
                Race {
                    time: 7,
                    distance: 9
                },
                Race {
                    time: 15,
                    distance: 40
                },
                Race {
                    time: 30,
                    distance: 200
                },
            ]
        )
    }

    #[test]
    fn test_boat() {
        let boat1 = Boat::new(1);
        assert_eq!(boat1.race(7), 6);

        let boat2 = Boat::new(2);
        assert_eq!(boat2.race(7), 10);

        let boat3 = Boat::new(3);
        assert_eq!(boat3.race(7), 12);

        let boat4 = Boat::new(4);
        assert_eq!(boat4.race(7), 12);

        let boat5 = Boat::new(5);
        assert_eq!(boat5.race(7), 10);

        let boat6 = Boat::new(6);
        assert_eq!(boat6.race(7), 6);

        let boat7 = Boat::new(7);
        assert_eq!(boat7.race(7), 0);
    }

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(EXAMPLE_INPUT), 288);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 1155175);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(EXAMPLE_INPUT), 71503);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 35961505);
    }
}
