#[derive(Copy, Clone)]
pub enum ServiceScope {
    Singleton,
    Request,
    AlwaysNew
}