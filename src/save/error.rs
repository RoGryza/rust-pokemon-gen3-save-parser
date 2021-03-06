use std::io;

use quick_error::{quick_error, Context};

quick_error! {
    #[derive(Debug)]
    pub enum LoadSaveError {
        CorruptData(msg: String) {
            display("Corrupt save data: {}", msg)
        }
        Io(msg: String, err: io::Error) {
            source(err)
            display("I/O Error (maybe corrupt save file): {} ({})", msg, err)
        }
    }
}

pub type LoadSaveResult<T> = Result<T, LoadSaveError>;

impl<S: Into<String>> From<Context<S, io::Error>> for LoadSaveError {
    fn from(ctx: Context<S, io::Error>) -> Self {
        LoadSaveError::Io(ctx.0.into(), ctx.1)
    }
}
