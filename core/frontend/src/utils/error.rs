use thiserror::Error;

#[derive(Error, Debug)]
pub enum GenericError {
    #[error("Couldn't cast the event target.")]
    DynCastError,

    #[error("Couldn't find the value in a JsObject.")]
    Undefined,
}
