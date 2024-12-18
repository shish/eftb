use rocket::http::ContentType;
use rocket::http::Status;
use rocket::request::Request;
use rocket::response::{self, Responder, Response};
use std::io::Cursor;
//use core::resp::Error;

//#[derive(Error)]
#[derive(Debug, Clone)]
pub struct CustomError(pub Status, pub String);

impl<'r> Responder<'r, 'static> for CustomError {
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {
        Response::build()
            .status(self.0)
            .header(ContentType::Text)
            .sized_body(self.1.len(), Cursor::new(self.1))
            .ok()
    }
}
