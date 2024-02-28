pub trait ICancellationToken {
    fn is_cancelled(&self) -> bool;
    fn cancel(&self);
}