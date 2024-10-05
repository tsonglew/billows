use actix_web::{web, HttpResponse};

pub fn register_urls(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(
        web::resource("/search")
            .route(web::get().to(|| async { HttpResponse::Ok().body("Hello World!") })),
    );
}
