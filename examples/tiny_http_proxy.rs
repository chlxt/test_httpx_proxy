
use tiny_http::{Server, Response};

fn main()
{

let server = Server::http("0.0.0.0:8000").unwrap();

// loop {
//     // blocks until the next request is received
//     let request = match server.recv() {
//         Ok(rq) => rq,
//         Err(e) => { println!("error: {}", e); break }
//     };
// }

for request in server.incoming_requests() {
    println!("received request! method: {:?}, url: {:?}, headers: {:?}",
        request.method(),
        request.url(),
        request.headers()
    );

    let response = Response::from_string("hello world");
    request.respond(response);
}

}