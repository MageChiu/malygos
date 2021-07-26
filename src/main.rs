/*
 * @Author: Charles
 * @LastEditors: Charles
 * @Description: file content
 */
extern crate iron;
#[macro_use]
extern crate mime;

use iron::prelude::*;
use iron::status;

fn main() {
    println!("Hello, world!");
    println!("serving on thpp://localhost:8200");
    Iron::new(get_from).http("localhost:8200").unwrap();
}

fn get_from(_request: &mut Request) -> IronResult<Response> {
    let mut response = Response::new();

    response.set_mut(status::Ok);
    response.set_mut(mime!(Text/Html; Charset=Utf8));
    response.set_mut(
        r#"
        <title> test </title>
    
    "#,
    );

    Ok(response)
}
