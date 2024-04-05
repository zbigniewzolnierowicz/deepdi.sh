#[derive(thiserror::Error, Debug)]
pub enum ValidationError {
    #[error("The field {0} was empty")]
    EmptyField(&'static str),

    #[error("Field {0} does not match any of: {1:?}")]
    DoesNotMatch(&'static str, &'static [&'static str]),

    #[error(transparent)]
    Unknown(#[from] eyre::Error),
}
