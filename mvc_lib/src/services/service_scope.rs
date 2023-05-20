

// the scope of the service instance tells the service collection how to create the service instance
// and when to let go or dispose of it.
#[derive(Copy, Clone, Debug)]
pub enum ServiceScope {
    // the service instance is created once and then held onto for the lifetime of the application.
    Singleton,
    // the service instance is created once per host and then disposed of at the end of the host's lifetime.
    Host,
    // the service instance is created once per request and then disposed of at the end of the request.
    Request,
    // the service instance is created once per area and then disposed of at the end of the area's lifetime.
    Area,
    // the service instance is created once per controller and then disposed of at the end of the controller's lifetime.
    Controller,
    // the service instance is created once per scope and then disposed of at the end of the scope.
    Scope,
    // the service instance is created per call to the service collection.
    AlwaysNew
}