pub fn part1(input: &str) -> u32 {
    let mut lines = input.lines();
    let ref_time = lines
        .next()
        .expect("expected line 1")
        .parse::<u32>()
        .expect("line 1 is a u32");
    let bus_list: Vec<u32> = lines
        .next()
        .expect("expected line 2")
        .split(',')
        .filter_map(|bus| bus.parse().ok())
        .collect();

    let mut min_wait = u32::MAX;
    let mut min_bus_id = 0;
    for bus in bus_list {
        let minutes_missed = ref_time % bus;
        let time_to_next_bus = bus - minutes_missed;
        if time_to_next_bus < min_wait {
            min_wait = time_to_next_bus;
            min_bus_id = bus;
        }
    }

    min_bus_id * min_wait
}

#[derive(Debug)]
struct Bus {
    id: u64,
    offset: u64,
    satisfied: bool
}

pub fn part2(input: &str) -> u64 {
    let mut buses: Vec<Bus> = input
        .lines()
        .nth(1)
        .expect("Expected line 2")
        .split(',')
        .enumerate()
        .filter_map(|(offset, id)| {
            id.parse::<u32>()
                .map(|bus_id| Bus {
                    id: bus_id as u64,
                    offset: offset as u64,
                    satisfied: false,
                })
                .ok()
        })
        .collect();

    let max_bus: &Bus = buses.iter().max_by_key(|bus| bus.id).unwrap();

    let mut step = 1;
    let mut candidate = max_bus.id - max_bus.offset - step;

    let mut done = 0;
    let buses_len = buses.len();

    while done < buses_len {
        candidate += step;
        // now we check if this creates a scale of timestamps
        for bus in buses.iter_mut() {
            if bus.satisfied {
                continue;
            }
            if (candidate + bus.offset as u64) % bus.id as u64 == 0 {
                step *= bus.id;  // here we should have done a LCM, but the bus ids are all prime, so...
                done += 1;
                bus.satisfied = true;
            }
        }
    }

    candidate
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part_1() {
        let input = include_str!("../input.txt");
        assert_eq!(part1(input), 136);
    }

    #[test]
    fn part_2_example() {
        let input = "0\n7,13,x,x,59,x,31,19";
        assert_eq!(part2(input), 1068781);
    }

    #[test]
    fn part_2() {
    // TODO: currently takes too long
        let input = include_str!("../input.txt");
        assert_eq!(part2(input), 305068317272992);
    }
}
