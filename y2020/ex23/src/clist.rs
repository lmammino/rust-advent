use std::ops::Index;

#[derive(Debug)]
pub struct CList<const N: usize> {
    values: [usize; N],
}

impl<const N: usize> CList<N> {}

impl<const N: usize> From<Vec<usize>> for CList<N> {
    fn from(v: Vec<usize>) -> Self {
        let mut values = [0; N];

        for i in 0..(v.len() - 1) {
            values[v[i]] = v[i + 1];
            println!("{} -> {}", v[i], v[i + 1])
        }

        for i in v.len()..N {
            values[i] = i + 1;
            println!("{} -> {}", i, i + 1)
        }

        values[N - 1] = v[0];
        println!("{} -> {}", N - 1, v[0]);

        CList { values }
    }
}

impl<const N: usize> Index<usize> for CList<N> {
    type Output = usize;

    fn index(&self, index: usize) -> &Self::Output {
        &self.values[index]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_vec() {
        let v: Vec<usize> = vec![3, 2, 4, 1, 5];
        let list: CList<6> = v.into();
        assert_eq!(list[3], 2);
        assert_eq!(list[2], 4);
        assert_eq!(list[4], 1);
        assert_eq!(list[1], 5);
        assert_eq!(list[5], 3);
        assert_eq!(list[0], 0);
    }

    #[test]
    fn test_from_vec_with_extra_space() {
        let v: Vec<usize> = vec![3, 2, 4, 1, 5];
        let list: CList<11> = v.into();
        dbg!(&list);
        assert_eq!(list[3], 2);
        assert_eq!(list[2], 4);
        assert_eq!(list[4], 1);
        assert_eq!(list[1], 5);
        assert_eq!(list[5], 6);
        assert_eq!(list[6], 7);
        assert_eq!(list[8], 9);
        assert_eq!(list[10], 3);
        assert_eq!(list[0], 0);
    }
}
