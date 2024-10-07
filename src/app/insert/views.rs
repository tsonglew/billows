use actix_web::{HttpResponse, HttpResponseBuilder};

pub async fn search_view() -> HttpResponse {
    return HttpResponse::Ok().body("hello insert!");
}
