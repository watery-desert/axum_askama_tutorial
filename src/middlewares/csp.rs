use axum::{
    extract::Request, http::header::{CONTENT_SECURITY_POLICY, REFERRER_POLICY, STRICT_TRANSPORT_SECURITY, X_CONTENT_TYPE_OPTIONS, X_FRAME_OPTIONS, X_XSS_PROTECTION}, middleware::Next, response::Response,
};

pub async fn secure_headers(req: Request, next: Next) -> Response {
    let csp_value = "default-src 'self';\
        style-src 'self' cdnjs.cloudflare.com fonts.googleapis.com;\
        font-src fonts.gstatic.com;\
    ";

    let mut res = next.run(req).await;

    res.headers_mut()
        .insert(CONTENT_SECURITY_POLICY, csp_value.parse().unwrap());

    res.headers_mut()
        .insert(X_FRAME_OPTIONS, "DENY".parse().unwrap());

    res.headers_mut()
        .insert(X_XSS_PROTECTION, "0".parse().unwrap());

    res.headers_mut().insert(
        STRICT_TRANSPORT_SECURITY,
        "max-age=31536000".parse().unwrap(),
    );

    res.headers_mut()
        .insert(X_CONTENT_TYPE_OPTIONS, "nosniff".parse().unwrap());

    res.headers_mut()
        .insert(REFERRER_POLICY, "origin-when-cross-origin".parse().unwrap());

    res
}
