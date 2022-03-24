#[derive(Debug)]
pub struct Bot {
    _token: String,
}

impl Bot {
    pub fn new<T>(token: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            _token: token.into(),
        }
    }
}
