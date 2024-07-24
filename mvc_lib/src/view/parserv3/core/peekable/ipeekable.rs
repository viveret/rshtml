pub trait IPeekable<T> {
    fn peek_nth(&self, i: usize) -> Option<T>;
    fn peek(&self) -> Option<T>;
    fn next(&self) -> Option<T>;
    fn to_vec(&self) -> Vec<T>;
}