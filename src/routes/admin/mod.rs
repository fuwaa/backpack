use actix_web::{web, Scope};

pub mod registration_key;

pub fn get_routes(invite_only: bool) -> Scope {
    let scope = web::scope("/admin");

    if invite_only {
        scope.service(registration_key::get_routes())
    } else {
        scope
    }
}
