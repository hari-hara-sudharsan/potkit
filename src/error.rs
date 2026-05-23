use thiserror::Error;

#[derive(Error, Debug)]
#[allow(dead_code)]
pub enum PotkitError {
    #[error("Feature not implemented yet")]
    NotImplemented,
}
