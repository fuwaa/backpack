use actix_http::StatusCode;
use actix_web::{delete, get, post, web, HttpResponse, Responder, Scope};

use crate::{
    database::entity::registration_keys,
    internal::{
        auth::{auth_role, Auth},
        response::Response,
        validate_paginate,
    },
    models::{
        admin::registration_key::{RegistrationKeyData, RegistrationKeyParams},
        MessageResponse, Page,
    },
    state::State,
};

use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, Set};

pub fn get_routes() -> Scope {
    web::scope("/registrationKey")
        .service(get_one)
        .service(list)
        .service(delete)
        .service(create)
}

#[post("")]
async fn create(
    state: web::Data<State>,
    user: Auth<auth_role::Admin>,
    query: web::Query<RegistrationKeyParams>,
) -> Response<impl Responder> {
    Ok(HttpResponse::Ok().json(RegistrationKeyData::from(
        registration_keys::ActiveModel {
            iss_user: Set(user.id.to_owned()),
            uses_left: Set(query.max_uses.unwrap_or(1)),
            ..Default::default()
        }
        .insert(&state.database)
        .await?,
    )))
}

#[get("/list/{page_number}")]
async fn list(
    state: web::Data<State>,
    page_number: web::Path<usize>,
    _user: Auth<auth_role::Admin>,
) -> Response<impl Responder> {
    let paginator = registration_keys::Entity::find().paginate(&state.database, 25);

    let pages = paginator.num_pages().await?;
    if let Some(err) = validate_paginate(*page_number, pages) {
        return Ok(err.http_response());
    }

    Ok(HttpResponse::Ok().json(Page {
        page: *page_number,
        pages,
        list: paginator
            .fetch_page(*page_number - 1)
            .await?
            .iter()
            .map(|model| RegistrationKeyData::from(model.to_owned()))
            .collect(),
    }))
}

#[get("/{registration_id}")]
async fn get_one(
    state: web::Data<State>,
    registration_id: web::Path<String>,
    _user: Auth<auth_role::Admin>,
) -> Response<impl Responder> {
    Ok(
        match registration_keys::Entity::find_by_id(registration_id.to_string())
            .one(&state.database)
            .await?
        {
            Some(v) => HttpResponse::Ok().json(RegistrationKeyData::from(v)),
            None => MessageResponse::new(StatusCode::NOT_FOUND, "Registration key not found")
                .http_response(),
        },
    )
}

#[delete("/{registration_id}")]
async fn delete(
    state: web::Data<State>,
    registration_id: web::Path<String>,
    _user: Auth<auth_role::Admin>,
) -> Response<impl Responder> {
    let result = registration_keys::Entity::delete_many()
        .filter(registration_keys::Column::Id.eq(registration_id.to_string()))
        .exec(&state.database)
        .await?;

    Ok(if result.rows_affected > 0 {
        MessageResponse::new(
            StatusCode::OK,
            &format!("Registration key ({}) was deleted", registration_id),
        )
    } else {
        MessageResponse::new(
            StatusCode::NOT_FOUND,
            &format!("Registration key ({}) not found", registration_id),
        )
    })
}
