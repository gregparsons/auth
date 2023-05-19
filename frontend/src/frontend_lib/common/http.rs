//! http.rs

use actix_web::{HttpResponse};

/// 302 redirect to the relative root "/"
/// authorization: TBD
pub async fn redirect_home()->HttpResponse{
    tracing::debug!("[redirect_home]");
    // redirect to home via 302
    // https://docs.rs/actix-web/latest/actix_web/http/struct.StatusCode.html#associatedconstant.FOUND
    // https://www.rfc-editor.org/rfc/rfc7231#section-6.4.3
    HttpResponse::Found()
        .append_header(("location", "/"))
        .append_header(("Cache-Control", "no-store"))
        .finish()

}
