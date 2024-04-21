#[derive(thiserror::Error, Debug)]
pub enum ValidationError {
    #[error("The fields {0:?} was empty")]
    EmptyField(Vec<&'static str>),

    #[error("Field {0} does not match any of: {1:?}")]
    DoesNotMatch(&'static str, &'static [&'static str]),

    #[error(transparent)]
    Unknown(#[from] eyre::Error),
}
