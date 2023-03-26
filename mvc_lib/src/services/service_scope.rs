#[derive(Copy, Clone, Debug)]
pub enum ServiceScope {
    Singleton,
    Request,
    AlwaysNew
}