pub struct By3Iter<I: Iterator> {
    inner: I,
}

impl<I: Iterator> By3Iter<I> {
    pub fn new(inner: I) -> Self {
        Self { inner }
    }
}

impl<I: Iterator> Iterator for By3Iter<I> {
    type Item = (I::Item, I::Item, I::Item);

    fn next(&mut self) -> Option<Self::Item> {
        let i1 = self.inner.next();
        let i2 = self.inner.next();
        let i3 = self.inner.next();

        match (i1, i2, i3) {
            (Some(i1), Some(i2), Some(i3)) => Some((i1, i2, i3)),
            _ => None,
        }
    }
}
