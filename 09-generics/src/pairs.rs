//! Generic STRUCT: holds two values of the same type `T`.

pub struct Pair<T> {
    pub a: T,
    pub b: T,
}

// `larger` only EXISTS when `T` can be ordered (PartialOrd) and copied out by
// value (Copy). Pairs of non-comparable types simply won't have this method.
impl<T: PartialOrd + Copy> Pair<T> {
    pub fn larger(&self) -> T {
        if self.a >= self.b { self.a } else { self.b }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_of_pair() {
        let p = Pair { a: 10u64, b: 25 };
        assert_eq!(p.larger(), 25);
    }
}
