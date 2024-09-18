pub trait ICancellationToken {
    fn is_cancelled(&self) -> bool;
    fn get_cancelled_reason(&self) -> String;
    fn cancel(&self);
    fn cancel_with_reason(&self, reason: String);
}