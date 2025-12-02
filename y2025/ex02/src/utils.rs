pub(crate) fn is_id_invalid(id: u64) -> bool {
    let digits: Vec<u8> = id
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect();

    digits.len() % 2 == 0 && digits[0..digits.len() / 2] == digits[digits.len() / 2..]
}

pub(crate) fn is_id_invalid_p2(id: u64) -> bool {
    let digits: Vec<u8> = id
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect();

    let len = digits.len();
    if len > 1 {
        for i in 1..len {
            if len % i == 0 {
                // needs to split the string in i-sized chunks
                let chunks = digits.chunks(i);
                let first_chunk = chunks.clone().next().unwrap();
                let different_chunk = chunks.skip(1).find(|chunk| *chunk != first_chunk);
                if different_chunk.is_none() {
                    return true;
                }
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_id_invalid() {
        assert!(is_id_invalid(11));
        assert!(is_id_invalid(22));
        assert!(is_id_invalid(99));
        assert!(is_id_invalid(1010));
        assert!(is_id_invalid(1188511885));
        assert!(is_id_invalid(222222));
        assert!(is_id_invalid(446446));
        assert!(is_id_invalid(38593859));
    }

    #[test]
    fn test_is_id_invalid_p2() {
        assert!(is_id_invalid_p2(11));
        assert!(is_id_invalid_p2(22));
        assert!(is_id_invalid_p2(99));
        assert!(is_id_invalid_p2(111));
        assert!(is_id_invalid_p2(999));
        assert!(is_id_invalid_p2(1010));
        assert!(is_id_invalid_p2(1188511885));
        assert!(is_id_invalid_p2(222222));
        assert!(is_id_invalid_p2(446446));
        assert!(is_id_invalid_p2(38593859));
        assert!(is_id_invalid_p2(565656));
        assert!(is_id_invalid_p2(824824824));
        assert!(is_id_invalid_p2(2121212121));
        assert!(!is_id_invalid_p2(112));
        assert!(!is_id_invalid_p2(1123));
    }
}
