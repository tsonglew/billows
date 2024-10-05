pub mod insert;
pub mod search;

pub fn register_urls(cfg: &mut actix_web::web::ServiceConfig) {
    insert::urls::register_urls(cfg);
    search::urls::register_urls(cfg);
}
