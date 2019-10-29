use log::error;
use std::error::Error as StdError;

struct SourceIter<'a> {
    current: Option<&'a (dyn StdError + 'static)>,
}
impl<'a> Iterator for SourceIter<'a> {
    type Item = &'a (dyn StdError + 'static);
    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;
        self.current = self.current.and_then(StdError::source);
        current
    }
}

pub fn source_iter(error: &impl StdError) -> impl Iterator<Item = &(dyn StdError + 'static)> {
    SourceIter {
        current: error.source(),
    }
}

pub trait ResultExt<T> {
    fn unwrap_display(self) -> T;
    fn unwrap_display_err(self) -> String;
    fn unwrap_log(self) -> Option<T>;
}

impl<T, E> ResultExt<T> for Result<T, E>
where
    E: StdError,
{
    fn unwrap_display(self) -> T {
        match self {
            Err(err) => {
                let mut cause = format!("{}", err);
                for source in source_iter(&err) {
                    cause += &format!("\ncaused by: {}", source);
                }
                panic!("\n{}", cause)
            }
            Ok(o) => o,
        }
    }

    fn unwrap_display_err(self) -> String {
        if let Err(err) = self {
            let mut cause = format!("{}", err);
            for source in source_iter(&err) {
                cause += &format!("\ncaused by: {}", source);
            }
            cause
        } else {
            panic!("Expected error")
        }
    }

    fn unwrap_log(self) -> Option<T> {
        match self {
            Err(err) => {
                error!("{}", err);
                for source in source_iter(&err) {
                    error!("caused by: {}", source);
                }
                None
            }
            Ok(o) => Some(o),
        }
    }
}
