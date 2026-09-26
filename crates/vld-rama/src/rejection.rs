//! Shared rejection type for all `Vld*` extractors.

use rama::http::service::web::response::IntoResponse;
use rama::http::{header, Response, StatusCode};

/// Rejection returned when validation (or body parsing) fails.
///
/// Converts to a **422 Unprocessable Entity** JSON response via
/// [`vld_http_common::format_vld_error`].
#[derive(Debug)]
pub struct VldRejection {
    error: vld::error::VldError,
}

impl VldRejection {
    /// Create a rejection from a [`vld::error::VldError`].
    #[must_use]
    pub fn from_vld(error: vld::error::VldError) -> Self {
        Self { error }
    }

    /// Create a rejection for a parse / transport error message.
    #[must_use]
    pub fn parse(message: impl Into<String>) -> Self {
        Self {
            error: vld::error::VldError::single(vld::error::IssueCode::ParseError, message.into()),
        }
    }

    /// Borrow the underlying validation error.
    #[must_use]
    pub fn error(&self) -> &vld::error::VldError {
        &self.error
    }
}

impl IntoResponse for VldRejection {
    fn into_response(self) -> Response {
        let body = vld_http_common::format_vld_error(&self.error);
        (
            StatusCode::UNPROCESSABLE_ENTITY,
            [(header::CONTENT_TYPE, "application/json")],
            body.to_string(),
        )
            .into_response()
    }
}

impl std::fmt::Display for VldRejection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Validation failed: {}", self.error)
    }
}

impl std::error::Error for VldRejection {}
