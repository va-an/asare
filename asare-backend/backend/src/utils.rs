pub trait ChainingExt: Sized {
    fn pipe<F, R>(self, f: F) -> R
    where
        F: FnOnce(Self) -> R,
    {
        f(self)
    }

    fn tap<F>(self, f: F) -> Self
    where
        F: FnOnce(),
    {
        f();
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
