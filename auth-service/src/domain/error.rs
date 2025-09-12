#[derive(Debug, PartialEq)]
pub enum AuthAPIError {
    UserAlreadyExists,
    InvalidCredentials,
    UnexpectedError,
    UnprocessableContent,
    IncorrectCredentials,
    MissingToken,
    InvalidToken,
}
