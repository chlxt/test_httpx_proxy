// [RFC 7231 / Connect]()
/*
[rust-web-framework-comparison](https://github.com/flosse/rust-web-framework-comparison)
[](https://www.lpalmieri.com/posts/2020-07-04-choosing-a-rust-web-framework-2020-edition/)
rust http server [higher-level]:
- [actix-web](https://actix.rs/)
- [rocket](https://rocket.rs/)
- [tower-web](https://github.com/carllerche/tower-web)
- [warp](https://github.com/seanmonstar/warp)
- [tide](https://github.com/http-rs/tide)
- [Thruster](https://github.com/thruster-rs/Thruster)
- [gotham](https://gotham.rs/)
- [nickel.rs](https://nickel-org.github.io/)
rust http server [lower-level]:
- [hyper](http://hyper.rs/)
- [tiny-http](https://github.com/tiny-http/tiny-http)
- [h2](https://github.com/hyperium/h2)
- [rust-http2](https://github.com/stepancheg/rust-http2)
*/

use tiny_http::{Server, Response};

fn main()
{

let server = Server::http("0.0.0.0:8000").unwrap();

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