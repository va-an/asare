// TODO: extract to "utils" crate
pub trait ChainingExt: Sized {
    fn pipe<F, R>(self, f: F) -> R
    where
        F: FnOnce(Self) -> R,
    {
        f(self)
    }

    fn tap<F>(self, f: F) -> Self
    where
        Self: Clone,
        F: FnOnce(Self),
    {
        f(self.to_owned());
        self
    }

    fn pipe_mut<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut Self),
    {
        f(&mut self);
        self
    }
}

impl<T> ChainingExt for T {}
