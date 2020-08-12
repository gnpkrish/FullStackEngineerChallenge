use actix_web::web;

mod handler;
pub mod model;

pub(super) fn route(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/graphql").route(web::post().to(handler::graphql)))
        .service(web::resource("/graphql/play").route(web::get().to(handler::playground)));
}
