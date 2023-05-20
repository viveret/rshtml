// define and implement the middleware
pub trait IRedirectHttpsMiddlewareService: IRequestMiddlewareService {
    fn redirect_to_https(self: &Self, request_context: Rc<dyn IRequestContext>, response_context: Rc<ResponseContext>) -> Result<MiddlewareResult, Box<dyn Error>>;
}

pub struct RedirectHttpsMiddlewareService {
    next: RefCell<Option<Rc<dyn IRequestMiddlewareService>>>,
}

impl RedirectHttpsMiddlewareService {
    pub fn new() -> Self {
        Self { next: RefCell::new(None) }
    }

    pub fn new_service(services: &dyn IServiceCollection) -> Vec<Box<dyn Any>> {
        vec![Box::new(Rc::new(Self::new()) as Rc<dyn IRequestMiddlewareService>)]
    }
}

impl IRedirectHttpsMiddlewareService for RedirectHttpsMiddlewareService {
    fn redirect_to_https(self: &Self, request_context: Rc<dyn IRequestContext>, response_context: Rc<ResponseContext>) -> Result<MiddlewareResult, Box<dyn Error>> {
        let mut url = request_context.get_url().clone();
        url.set_scheme("https");
        HttpRedirectResult::config_response(response_context, url.to_string());
        Ok(MiddlewareResult::OkBreak)
    }
}