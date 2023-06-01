use crate::http::http_body_content::ContentType;

pub trait IContentTypeResolver<T> {
    fn try_get(&self, content_type: ContentType) -> Option<T>;
}