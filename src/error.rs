use std::error::Error;

pub type ResolveResult<T> = Result<T,Box<ResolveError>>;

#[derive(Clone,Debug,Default)]
pub struct ResolveError {
    pub parent_type_name: &'static str,
    pub parent_id: String,
    pub child_type_name: &'static str,
    pub child_id: String,
    pub err_desc: String,
}
