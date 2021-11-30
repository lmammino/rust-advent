pub fn part1(input: &str) -> usize {
    let mut arr = [0; 10];
    let str_len = input.len();
    for i in 0..str_len {
        let a: usize = input[i..(i + 1)].parse().unwrap();
        let b: usize = input[((i + 1) % str_len)..(((i + 1) % str_len) + 1)]
            .parse()
            .unwrap();
        arr[a] = b;
    }
    let mut curr: usize = input[0..1].parse().unwrap();
    for _ in 0..100 {
        let curr_1 = arr[curr];
        let curr_2 = arr[curr_1];
        let curr_3 = arr[curr_2];
        let curr_4 = arr[curr_3];

        let mut dest_value = curr - 1;
        if dest_value == 0 {
            dest_value = 9
        }
        while dest_value == curr_1 || dest_value == curr_2 || dest_value == curr_3 {
            dest_value -= 1;
            if dest_value == 0 {
                dest_value = 9
            }
        }

        arr[curr] = curr_4;
        arr[curr_3] = arr[dest_value];
        arr[dest_value] = curr_1;
        curr = arr[curr];
    }

    let mut ret = 0;
    let mut p = arr[1];

    while p != 1 {
        ret *= 10;
        ret += p;
        p = arr[p];
    }

    ret
}

pub fn part2(input: &str) -> u64 {
    let mut arr = vec![0_usize; 1000001].into_boxed_slice();
    let str_len = input.len();
    for i in 0..str_len {
        let a: usize = input[i..(i + 1)].parse().unwrap();
        let b: usize = input[((i + 1) % str_len)..(((i + 1) % str_len) + 1)]
            .parse()
            .unwrap();
        arr[a] = b;
    }
    for i in 10..=1000000 {
        arr[i] = i + 1;
    }
    let mut curr: usize = input[0..1].parse().unwrap();
    let last: usize = input[(str_len - 1)..str_len].parse().unwrap();
    arr[last] = 10;
    arr[1000000] = curr;
    for _ in 0..10000000 {
        let curr_1 = arr[curr];
        let curr_2 = arr[curr_1];
        let curr_3 = arr[curr_2];
        let curr_4 = arr[curr_3];

        let mut dest_value = curr - 1;
        if dest_value == 0 {
            dest_value = 1000000
        }
        while dest_value == curr_1 || dest_value == curr_2 || dest_value == curr_3 {
            dest_value -= 1;
            if dest_value == 0 {
                dest_value = 1000000
            }
        }

        arr[curr] = curr_4;
        arr[curr_3] = arr[dest_value];
        arr[dest_value] = curr_1;
        curr = arr[curr];
    }

    arr[1] as u64 * arr[arr[1]] as u64
}

#[cfg(test)]
mod ex23_tests {
    use super::*;

    #[test]
    fn part_1() {
        let input = include_str!("../input.txt");
        assert_eq!(part1(input), 26354798);
    }

    #[test]
    fn part_2() {
        let input = include_str!("../input.txt");
        assert_eq!(part2(input), 166298218695);
    }
}
