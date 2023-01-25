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

        if i1.is_some() && i2.is_some() && i3.is_some() {
            return Some((i1.unwrap(), i2.unwrap(), i3.unwrap()));
        }

        None
    }
}
