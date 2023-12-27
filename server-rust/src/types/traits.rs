use crate::types::{BadRequest, Error};
use rocket::{
    http::{ContentType, Status},
    request::Request,
    response, Response,
};
use std::io::Cursor;

// Convert our custom Error type into HTTP responses
impl<'r> response::Responder<'r> for Error {
    fn respond_to(self, _: &Request) -> response::Result<'r> {
        let body = BadRequest::new(self);
        let body = serde_json::to_string(&body).expect("to BadRequest serialize");
        Ok(Response::build()
            .status(Status::BadRequest)
            .header(ContentType::JSON)
            .sized_body(Cursor::new(body))
            .finalize())
    }
}
