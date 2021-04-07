use crate::shortened_names::expand_name;

pub struct ValidationError {
    field: &'static str,
    message: String,
}

impl ValidationError {
    pub fn new<S: Into<String>>(field: &'static str, message: S) -> Self {
        ValidationError {
            field,
            message: message.into(),
        }
    }

    pub fn from_display<S: Display>(field: &'static str, message: S) -> Self {
        ValidationError {
            field,
            message: message.to_string(),
        }
    }
}
