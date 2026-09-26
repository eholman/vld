use crate::extract::headers_to_json;
use crate::rejection::VldRejection;
use rama::http::request::Parts;
use rama::http::service::web::extract::FromPartsStateRefPair;
use vld::schema::VldParse;

/// Rama extractor that validates **HTTP headers**.
///
/// Header names are normalised to snake_case for schema matching:
/// `Content-Type` → `content_type`, `X-Request-Id` → `x_request_id`.
pub struct VldHeaders<T>(pub T);

impl<T, State> FromPartsStateRefPair<State> for VldHeaders<T>
where
    T: VldParse + Send + Sync + 'static,
    State: Send + Sync + 'static,
{
    type Rejection = VldRejection;

    async fn from_parts_state_ref_pair(
        parts: &Parts,
        _state: &State,
    ) -> Result<Self, Self::Rejection> {
        let value = headers_to_json(&parts.headers);
        let parsed = T::vld_parse_value(&value).map_err(VldRejection::from_vld)?;
        Ok(VldHeaders(parsed))
    }
}
