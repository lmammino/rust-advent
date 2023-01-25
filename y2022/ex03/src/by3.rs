pub struct By3Iter<I: Iterator>(I);

impl<I: Iterator> By3Iter<I> {
    pub fn new(inner: I) -> Self {
        By3Iter(inner)
    }
}

impl<I: Iterator> Iterator for By3Iter<I> {
    type Item = (I::Item, I::Item, I::Item);

    fn next(&mut self) -> Option<Self::Item> {
        let i1 = self.0.next();
        let i2 = self.0.next();
        let i3 = self.0.next();

        match (i1, i2, i3) {
            (Some(i1), Some(i2), Some(i3)) => Some((i1, i2, i3)),
            _ => None,
        }
    }
}
