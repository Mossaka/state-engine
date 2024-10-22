use serde::{Deserialize, Serialize};
use serde_json;
use anyhow;
use spin_sdk::http::{HeaderValue, Request, Response};
use spin_sdk::http_component;
use spin_sdk::key_value::Store;

/// A simple Spin HTTP component.
#[http_component]
fn handle_state_put(req: Request) -> anyhow::Result<Response> {
    println!("Handling request to {:?}", req.header("spin-full-url"));
    let store = req
        .header("spin-path-match-state-store-name")
        .ok_or_else(|| anyhow::anyhow!("Missing store name"))?
        .clone()
        .into_utf8_lossy();
    let store = Store::open(&store).unwrap();
    let method = req.method();
    let content_type = req.header("content-type");

    // Check if the content type is JSON
    if content_type != Some(&HeaderValue::string("application/json".to_string())) {
        return Ok(Response::builder()
            .status(400)
            .header("content-type", "text/plain")
            .body("Bad Request: Content-Type must be application/json")
            .build());
    }

    match method {
        spin_sdk::http::Method::Post => handle_state_post(store, req.into_body()),
        _ => Ok(Response::builder()
            .status(405)
            .header("content-type", "text/plain")
            .body("Method Not Allowed")
            .build()),
    }
}

fn handle_state_post(store: Store, body: Vec<u8>) -> anyhow::Result<Response> {
    let schemas: Vec<Schema> = serde_json::from_slice(&body)?;
    for schema in schemas {
        let key = schema.key;
        let val = schema.value;
        let value_bytes = serde_json::to_vec(&val)?;
        store.set(&key, &value_bytes)?;
    }
    Ok(Response::builder()
        .status(200)
        .header("content-type", "text/plain")
        .body("OK")
        .build())
}

#[derive(Serialize, Deserialize)]
struct Schema {
    key: String,
    value: serde_json::Value,
}
