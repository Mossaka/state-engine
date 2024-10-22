use spin_sdk::http::{IntoResponse, Request, Response};
use spin_sdk::http_component;
use spin_sdk::key_value::Store;
/// A simple Spin HTTP component.
#[http_component]
fn handle_state_get_delete(req: Request) -> anyhow::Result<Response> {
    println!("Handling request to {:?}", req.header("spin-full-url"));
    let store = req
        .header("spin-path-match-state-store-name")
        .ok_or_else(|| anyhow::anyhow!("Missing store name"))?
        .clone()
        .into_utf8_lossy();
    let key = req
        .header("spin-path-match-key")
        .ok_or_else(|| anyhow::anyhow!("Missing key"))?
        .clone()
        .into_utf8_lossy();
    println!("Store: {:?}, Key: {:?}", store, key);

    let store = Store::open(&store).unwrap();
    let method = req.method();

    match method {
        spin_sdk::http::Method::Get => handle_state_get(store, &key),
        spin_sdk::http::Method::Delete => handle_state_delete(store, &key),
        _ => Ok(Response::builder()
            .status(405)
            .header("content-type", "text/plain")
            .body("Method Not Allowed")
            .build()),
    }
}

fn handle_state_get(store: Store, key: &str) -> anyhow::Result<Response> {
    let res = store.get(key);
    match res {
        Ok(value) => match value {
            Some(v) => Ok(Response::builder()
                .status(200)
                .header("content-type", "text/plain")
                .body(v)
                .build()),
            None => Ok(Response::builder()
                .status(404)
                .header("content-type", "text/plain")
                .body("")
                .build()),
        },
        Err(_) => Ok(Response::builder()
            .status(500)
            .header("content-type", "text/plain")
            .body("")
            .build()),
    }
}

fn handle_state_delete(store: Store, key: &str) -> anyhow::Result<Response> {
    match store.delete(key) {
        Ok(_) => Ok(Response::builder()
            .status(204)
            .header("content-type", "text/plain")
            .body("")
            .build()),
        Err(_) => Ok(Response::builder()
            .status(500)
            .header("content-type", "text/plain")
            .body("")
            .build()),
    }
}
