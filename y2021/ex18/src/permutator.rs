pub struct Permutator<'a, T> {
    i: usize,
    j: usize,
    expressions: &'a Vec<T>,
}

impl<'a, T> Permutator<'a, T> {
    pub fn new(expressions: &'a Vec<T>) -> Self {
        Permutator {
            i: 0,
            j: 0,
            expressions,
        }
    }
}

impl<'a, T> Iterator for Permutator<'a, T> {
    type Item = (&'a T, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.i == self.j {
            self.i += 1;

            if self.i == self.expressions.len() {
                self.i = 0;
                self.j += 1;
            }
        }

        if self.j == self.expressions.len() {
            return None;
        }

        let left = self.expressions.get(self.i).unwrap();
        let right = self.expressions.get(self.j).unwrap();

        self.i += 1;

        if self.i == self.expressions.len() {
            self.i = 0;
            self.j += 1;
        }

        Some((left, right))
    }
}
