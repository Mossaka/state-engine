use spin_sdk::{
    http::{IntoResponse, Request, RequestBuilder, Response},
    http_component,
};


#[http_component]
async fn handle_request(req: Request) -> anyhow::Result<impl IntoResponse> {
    let mut request = req.clone();
    request.set_uri(format!("http://localhost:3000{}", req.path_and_query().unwrap_or("/")));

    // Send the request and await the response
    let response: Response = spin_sdk::http::send(request).await?;

    // Use the outbound response body
    let response_len = response.body().len();

    // Return the response to the inbound request
    Ok(Response::builder()
        .status(200)
        .header("content-type", "text/plain")
        .body(format!("The test page was {response_len} bytes"))
        .build())
}