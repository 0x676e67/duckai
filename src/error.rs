#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    IOError(#[from] std::io::Error),

    #[error(transparent)]
    ParseIntError(#[from] std::num::ParseIntError),

    #[error(transparent)]
    NetworkParseError(#[from] cidr::errors::NetworkParseError),

    #[error(transparent)]
    AddressParseError(#[from] std::net::AddrParseError),

    #[cfg(target_family = "unix")]
    #[error(transparent)]
    NixError(#[from] nix::Error),

    #[error(transparent)]
    SerdeYamlError(#[from] serde_yaml::Error),

    #[error(transparent)]
    RequestError(#[from] rquest::Error),

    #[error(transparent)]
    LogParseError(#[from] tracing_subscriber::filter::ParseError),

    #[error(transparent)]
    LogSetGlobalDefaultError(#[from] tracing::subscriber::SetGlobalDefaultError),

    #[error(transparent)]
    ResolveError(#[from] hickory_resolver::error::ResolveError),

    #[error(transparent)]
    JsonExtractorRejection(#[from] axum::extract::rejection::JsonRejection),

    #[error(transparent)]
    AxumHttpError(#[from] axum::http::Error),

    #[error("Missing or invalid 'x-vqd-4' header")]
    MissingHeader,

    #[error("{0}")]
    BadRequest(String),
}
