use itertools::Itertools;

#[derive(Debug)]
pub struct CLList {
    data: Box<[usize]>,
}

impl CLList {
    pub fn from_iter(iter: impl Iterator<Item = usize> + Clone, first: usize) -> Self {
        let mut data = vec![0; 1_000_001];

        let mut last = 0;
        for (a, b) in iter.into_iter().tuple_windows() {
            data[a] = b;
            last = b;
        }
        data[last] = first;

        CLList {
            data: data.into_boxed_slice(),
        }
    }

    pub fn next(&self, i: usize) -> usize {
        self.data[i]
    }

    pub fn pop3_after(&mut self, i: usize) -> [usize; 3] {
        let one = self.next(i);
        let two = self.next(one);
        let three = self.next(two);

        self.data[i] = self.next(three);

        [one, two, three]
    }

    pub fn push3_after(&mut self, i: usize, elements: [usize; 3]) {
        let next_target = self.next(i);

        self.data[i] = elements[0];
        self.data[elements[2]] = next_target;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constructor() {
        let data: [usize; 6] = [1, 2, 3, 4, 5, 6];
        let mut list: CLList = CLList::from_iter(data.into_iter(), 1);
        assert_eq!(list.next(3), 4);
        assert_eq!(list.next(6), 1);

        assert_eq!(list.pop3_after(2), [3, 4, 5]);
        assert_eq!(list.next(2), 6);

        list.push3_after(1, [3, 4, 5]);
        assert_eq!(list.next(1), 3);
        assert_eq!(list.next(5), 2);
    }
}
