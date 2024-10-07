use actix_web::{web, HttpResponse};

use super::views;

pub fn register_urls(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(web::scope("/insert").route("/reservation", web::post().to(views::search_view)));
}
