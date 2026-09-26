mod cookie;
mod form;
mod headers;
mod json;
mod path;
mod query;

pub use cookie::VldCookie;
pub use form::VldForm;
pub use headers::VldHeaders;
pub use json::VldJson;
pub use path::VldPath;
pub use query::VldQuery;

use vld_http_common::coerce_value;

/// Build a JSON object from HTTP headers.
///
/// Header names are normalised: `Content-Type` → `content_type`.
fn headers_to_json(headers: &rama::http::HeaderMap) -> serde_json::Value {
    let mut map = serde_json::Map::new();

    for (name, value) in headers.iter() {
        let key = name.as_str().replace('-', "_");
        if let Ok(v) = value.to_str() {
            map.insert(key, coerce_value(v));
        }
    }

    serde_json::Value::Object(map)
}
