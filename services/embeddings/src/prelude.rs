use std::result::Result as StdResult;

use actix_web::HttpResponse;

pub type Result<T> = StdResult<T, HttpResponse>;