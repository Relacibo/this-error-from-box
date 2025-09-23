use thiserror::Error;
use this_error_from_box::this_error_from_box;
use std::rc::Rc;

#[derive(Error, Debug)]
#[this_error_from_box(Rc)]
pub enum MyError {
    #[error("Io error")]
    Io(#[from] Rc<std::io::Error>),
    #[error("Utf8 error")]
    Utf8(#[from] Rc<std::string::FromUtf8Error>),
    #[error("Other")]
    Other,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_io_error() {
        let io_err = std::io::Error::other("fail");
        let err: MyError = io_err.into();
        match err {
            MyError::Io(boxed) => assert_eq!(boxed.kind(), std::io::ErrorKind::Other),
            _ => panic!("Falsche Variante"),
        }
    }

    #[test]
    fn test_from_utf8_error() {
        let bytes = vec![0xff];
        let utf8_err = String::from_utf8(bytes).unwrap_err();
        let err: MyError = utf8_err.into();
        match err {
            MyError::Utf8(boxed) => assert_eq!(boxed.utf8_error().valid_up_to(), 0),
            _ => panic!("Falsche Variante"),
        }
    }
}
