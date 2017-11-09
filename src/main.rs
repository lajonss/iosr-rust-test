#[macro_use]
extern crate nickel;

use nickel::Nickel;
use nickel::HttpRouter;

fn main() {
    let mut server = Nickel::new();
    server.get(
        "/greeting",
        middleware! {|_reg, _resp|
        "Ahoj przygodo!"
    },
    );
    server.listen("0.0.0.0:8080").expect(
        "Error starting server.",
    );
}
