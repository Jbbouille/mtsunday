use warp::{self, path, Filter};
use std::path::PathBuf;

pub fn serve(certificate: Option<PathBuf>, key: Option<PathBuf>, secured: bool) {
    let hello = path!("hello" / String)
        .map(|name| format!("Hello, {}!", name));

    if secured {
        warp::serve(hello)
            .tls(certificate.expect(""), key.expect(""))
            .run(([0, 0, 0, 0], 443));
    } else {
        warp::serve(hello)
            .run(([127, 0, 0, 1], 8080));
    }
}