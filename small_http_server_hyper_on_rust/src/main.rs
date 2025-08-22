use hyper::{
    body::to_bytes,
    service::{make_service_fn, service_fn},
    Body, Method, Request, Response, Server, StatusCode,
};
use small_http_server_hyper_on_rust::validate_request;
use serde_json::json;
use std::convert::Infallible;

async fn handle_request(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    match (req.method(), req.uri().path()) {
        // Only handle POST /validate
        (&Method::POST, "/validate") => {
            let body_bytes = to_bytes(req.into_body()).await.unwrap_or_default();
            let body_str = String::from_utf8_lossy(&body_bytes);

            match validate_request(&body_str) {
                Ok(validated) => {
                    let resp_body = json!({
                        "status": "ok",
                        "message": "Valid request received",
                        "data": validated
                    });
                    Ok(Response::new(Body::from(resp_body.to_string())))
                }
                Err(err) => {
                    let resp_body = json!({
                        "status": "error",
                        "message": "Invalid request payload",
                        "error": err
                    });
                    let mut resp = Response::new(Body::from(resp_body.to_string()));
                    *resp.status_mut() = StatusCode::BAD_REQUEST;
                    Ok(resp)
                }
            }
        }

        // Handle other routes
        _ => {
            let mut resp = Response::new(Body::from("Not Found"));
            *resp.status_mut() = StatusCode::NOT_FOUND;
            Ok(resp)
        }
    }
}

#[tokio::main]
async fn main() {
    let addr = ([127, 0, 0, 1], 3000).into();

    let make_svc = make_service_fn(|_conn| async {
        Ok::<_, Infallible>(service_fn(handle_request))
    });

    let server = Server::bind(&addr).serve(make_svc);

    println!("Server running at http://{}", addr);

    if let Err(e) = server.await {
        eprintln!("Server error: {}", e);
    }
}