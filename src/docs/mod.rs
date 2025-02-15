use utoipa::openapi::security::HttpAuthScheme;
use utoipa::openapi::security::HttpBuilder;
use utoipa::openapi::security::SecurityScheme;
use utoipa::openapi::LicenseBuilder;
use utoipa::Modify;
use utoipa::OpenApi;

use crate::models::*;

use crate::routes;

/// Backpack API Documentation
#[derive(OpenApi)]
#[openapi(
    handlers(
		routes::info,
		routes::user::info,
		routes::user::settings,
		routes::user::create,
		routes::user::verify,
		routes::user::resend_verify,
		routes::file::upload,
        routes::file::stats,
        routes::file::list,
        routes::file::info,
        routes::file::delete_file,
        routes::application::token,
        routes::application::list,
        routes::application::info,
        routes::application::create,
        routes::application::delete,
        routes::auth::basic
	),
    components(
		AppInfo,
		MessageResponse,
		UserData,
		UserRole,
		UpdateUserSettings,
		UserCreateForm,
		UploadFile,
        FileData,
        FileStats,
        FilePage,
        ApplicationData,
        TokenResponse,
        ApplicationCreate,
        BasicAuthForm
	),
	tags(
		(name = "server", description = "Server information endpoints."),
		(name = "user", description = "User management endpoints."),
        (name = "file", description = "File management endpoints."),
        (name = "application", description = "Application and token management endpoints."),
        (name = "authentication", description = "User authentication endpoints.")
	),
    modifiers(&ApiDoc)
)]
pub struct ApiDoc;

/// Runtime modification for documentation
impl Modify for ApiDoc {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        // Authentication
        let components = openapi.components.as_mut().unwrap(); // we can unwrap safely since there already is components registered.
        components.add_security_scheme(
            "apiKey",
            SecurityScheme::Http(
                HttpBuilder::new()
                    .scheme(HttpAuthScheme::Bearer)
                    .bearer_format("JWT")
                    .description(Some(include_str!("ApiKey.md")))
                    .build(),
            ),
        );

        // License
        openapi.info.license = Some(
            LicenseBuilder::default()
                .name("MIT")
                .url(Some(
                    "https://github.com/JSH32/Backpack/blob/rewrite/LICENSE",
                ))
                .build(),
        )
    }
}
