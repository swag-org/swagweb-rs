use std::net::SocketAddrV4;

use crate::http::request::HttpRequest;

pub struct HttpContext {
    request: HttpRequest,
    path: String,
    socket_meta: SocketAddrV4
}
