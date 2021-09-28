use std::collections::HashMap;
use std::convert::Infallible;
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use std::str::FromStr;
use hyper::header::HeaderValue;
use hyper::{Body, Request, Response, Server, StatusCode};
use hyper::service::{make_service_fn, service_fn};
use shortcut::ShortcutRegistry;
use std::sync::{Arc};
use url::Url;

mod config;
mod shortcut;

fn extract_query_parameter(uri: &hyper::Uri) -> Option<String> {
    let uri = format!("http://0.0.0.0/{}", uri);
    let parsed_url = match Url::parse(&uri) {
        Ok(url) => url,
        Err(_) => return None,
    };
    let query_params: HashMap<_, String> =  parsed_url.query_pairs().into_owned().collect();
    match query_params.get("q") {
        Some(query) => Some(query.to_string()),
        None => None,
    }
}

async fn index(req: Request<Body>, shortcut_registry: Arc<ShortcutRegistry>) -> Result<Response<Body>, Infallible> {
    let mut response = Response::new(Body::empty());
    match (req.method(), extract_query_parameter(&*req.uri())) {
        (&hyper::Method::GET, Some(query)) => {
            let response = Response::new(Body::empty());
            let (mut parts, _body) = response.into_parts();
            parts.status = StatusCode::TEMPORARY_REDIRECT;
            parts.headers.append("Location", HeaderValue::from_str(&*shortcut_registry.match_query(&*query)).unwrap());
            Ok(Response::from_parts(parts, Body::empty()))
        }
        _ => {
            *response.body_mut() = Body::from("Please provide a search via /search?q=%s");
            Ok(response)
        },
    }
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install CTRL+C signal handler");
}


#[tokio::main]
async fn main() {
    let app_config = config::parse_args();
    let shortcut_registry = match shortcut::ShortcutRegistry::new(app_config.shortcuts_path, app_config.search_engine) {
        Ok(registry) => Arc::new(registry),
        Err(e) => {
            eprintln!("{}.", e);
            std::process::exit(1);
        }
    };
    let ip = Ipv4Addr::from_str(&*app_config.host).unwrap_or(Ipv4Addr::new(0, 0, 0, 0));
    let addr = SocketAddr::from(SocketAddrV4::new(
        ip,
        app_config.port,
    ));
    let server = Server::bind(&addr).serve(make_service_fn(move |_conn| {
        let shortcut_registry = shortcut_registry.clone();
        async move { Ok::<_, Infallible>(service_fn(move |req| index(req, shortcut_registry.clone()))) }
    }));

    println!("Listening for queries via http://{}:{}/search?q=%s", addr.ip(), addr.port());

    let graceful = server.with_graceful_shutdown(shutdown_signal());

    if let Err(e) = graceful.await {
        eprintln!("server error: {}", e)
    }
}

