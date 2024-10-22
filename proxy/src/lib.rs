use spin_sdk::{
    http::{IntoResponse, Request, RequestBuilder, Response},
    http_component,
};


#[http_component]
async fn handle_request(req: Request) -> anyhow::Result<impl IntoResponse> {

    // Create the outbound request object
    // let request = Request::builder()
    //     .method(req.method().clone())
    //     .uri(format!("http://localhost:3000{}", req.path_and_query().unwrap_or("/")))
    //     .headers(req.)
    //     .body(req.into_body())
    //     .build();

    // // print request
    // println!("Request method: {:?}", request.method());
    // println!("Request URI: {:?}", request.uri());
    // change req uri to http://localhost:3000

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