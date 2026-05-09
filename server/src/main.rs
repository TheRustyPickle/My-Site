use actix_files::Files;
use actix_web::web::{Data, Json, Path};
use actix_web::{App, HttpResponse, HttpServer, dev, web};
use app::App;
use chrono::{Days, Utc};
use dev::Service;
use leptos::config::get_configuration;
use leptos::prelude::*;
use leptos_actix::{LeptosRoutes, generate_route_list};
use leptos_meta::MetaTags;
use log::{LevelFilter, error, info};
use reqwest::Client;
use std::time::Duration;
use tokio::time::sleep;
use vial_shared::CreateSecretRequest;
use vial_shared::config::Config;
use vial_srv::db::{Handler, get_connection};
use vial_srv::errors::ServerError;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();

    pretty_env_logger::formatted_timed_builder()
        .format_timestamp_millis()
        .filter_level(LevelFilter::Info)
        .init();

    let conf = get_configuration(None).unwrap();

    let config = Config::default();

    let port = config.get_port();

    let address = config.get_address();

    let addr = format!("{address}:{port}");
    let addr_clone = addr.clone();

    let db_url = config.get_database_url_verified();
    let db_handler = get_connection(&db_url).await;

    // INFO: Add it add a later point perhaps
    // db_handler.clear_expired_days(30);

    tokio::spawn(ping_site());
    tokio::spawn(ping_api());

    HttpServer::new(move || {
        // Generate the list of routes in your Leptos App
        let routes = generate_route_list(App);
        let leptos_options = &conf.leptos_options;
        let site_root = leptos_options.site_root.clone().to_string();

        info!("listening on http://{addr_clone}");

        App::new()
            .wrap_fn(|req, srv| {
                let path = req.path();

                let ip = req
                    .connection_info()
                    .realip_remote_addr()
                    .unwrap_or("Unknown")
                    .to_string();

                let ignore_ip_list = [
                    "146.70.199.165",
                    "54.254.162.138",
                    "74.220.52.2",
                    "74.220.52.251",
                    "170.106.165.76",
                    "43.166.244.251",
                    "185.100.232.165",
                ];

                if !path.starts_with("/pkg/")
                    && !path.ends_with(".js")
                    && !path.ends_with(".js")
                    && !path.ends_with(".css")
                    && !path.ends_with(".wasm")
                    && !path.starts_with("/assets/")
                    && !path.starts_with("/favicon.ico")
                    && path != "/sw.js"
                    && !ignore_ip_list.contains(&ip.as_str())
                {
                    info!("Serving data for path: {path}. Request gotten from: {ip}",);
                }

                let fut = srv.call(req);
                async {
                    let res = fut.await?;
                    Ok(res)
                }
            })
            .app_data(Data::new(db_handler.clone()))
            .app_data(Data::new(config.clone()))
            .app_data(web::JsonConfig::default().limit(config.get_max_size_verified()))
            .service(
                web::scope("/api/secrets")
                    .route("/{id}", web::get().to(get_secret))
                    .route("", web::post().to(create_secret)),
            )
            .service(Files::new("/pkg", format!("{site_root}/pkg")))
            .service(Files::new("/assets", &site_root))
            .service(favicon)
            .service(robots)
            .leptos_routes(routes, {
                let leptos_options = leptos_options.clone();
                move || {
                    let leptos_options = leptos_options.clone();
                    view! {
                        <!DOCTYPE html>
                        <html lang="en" class="bg-gray-100 dark:bg-gray-900">
                            <head>
                                <meta charset="utf-8" />
                                <meta
                                    name="viewport"
                                    content="width=device-width, initial-scale=1"
                                />
                                <AutoReload options=leptos_options.clone() />
                                <HydrationScripts options=leptos_options.clone() />
                                <MetaTags />
                            </head>
                            <body class="bg-gray-100 dark:bg-gray-900">
                                <App />
                            </body>
                        </html>
                    }
                }
            })
            .app_data(web::Data::new(leptos_options.to_owned()))
    })
    .bind(&addr)?
    .run()
    .await
}

#[actix_web::get("favicon.ico")]
async fn favicon(
    leptos_options: actix_web::web::Data<leptos::config::LeptosOptions>,
) -> actix_web::Result<actix_files::NamedFile> {
    let leptos_options = leptos_options.into_inner();
    let site_root = &leptos_options.site_root;
    Ok(actix_files::NamedFile::open(format!(
        "{site_root}/favicon.ico"
    ))?)
}

#[actix_web::get("robots.txt")]
async fn robots(
    leptos_options: actix_web::web::Data<leptos::config::LeptosOptions>,
) -> actix_web::Result<actix_files::NamedFile> {
    let leptos_options = leptos_options.into_inner();
    let site_root = &leptos_options.site_root;
    Ok(actix_files::NamedFile::open(format!(
        "{site_root}/robots.txt"
    ))?)
}

async fn ping_site() {
    let client = Client::new();
    let url = "https://rustypickle.onrender.com/";

    info!("Pinger initialized");
    loop {
        let _ = client.get(url).send().await;
        sleep(Duration::from_secs(100)).await;
    }
}

async fn ping_api() {
    let client = Client::new();
    let url = "https://svp-dashboard.vercel.app/api/svp/cleanup";

    let auth_token = match std::env::var("CLEANUP_SECRET") {
        Ok(token) => token,
        Err(_) => {
            error!("CLEANUP_SECRET env var is not set");
            return;
        }
    };

    info!("API Pinger initialized");
    loop {
        let _ = client.post(url).bearer_auth(&auth_token).send().await;
        sleep(Duration::from_secs(60 * 5)).await;
    }
}

async fn get_secret(id: Path<String>, db_handler: Data<Handler>) -> HttpResponse {
    let id = id.into_inner();
    info!("Getting secret with id: {id}");

    db_handler
        .get_secret(&id)
        .await
        .map_or_else(server_error_to_response, |secret| {
            if let Some(secret) = secret {
                HttpResponse::Ok().json(secret)
            } else {
                HttpResponse::NotFound().body("secret not found")
            }
        })
}

async fn create_secret(
    db_handler: Data<Handler>,
    config: Data<Config>,
    payload: Json<CreateSecretRequest>,
) -> HttpResponse {
    let payload = payload.into_inner();

    if payload.expires_at.is_none() && payload.max_views.is_none() {
        info!("Secret expire and view count are empty");
        return server_error_to_response(ServerError::ViewAndExpireEmpty);
    }

    let max_size = config.get_max_size_verified();
    let max_day = config.get_max_days_verified();
    let max_view = config.get_max_views_verified();

    if payload.ciphertext.len() > max_size || payload.ciphertext.is_empty() {
        info!(
            "Payload too large. Max size is {max_size} bytes. Gotten {}",
            payload.ciphertext.len()
        );

        return HttpResponse::PayloadTooLarge()
            .body("Payload size is invalid. Max size is {MAX_SIZE} bytes");
    }

    if let Some(payload_day) = payload.expires_at {
        let max_naivetime = Utc::now().naive_utc() + Days::new(max_day as u64);

        if payload_day > max_naivetime || payload_day < Utc::now().naive_utc() {
            info!(
                "Payload day is invalid. Max day is {max_day}. Gotten {}",
                payload_day
            );

            return server_error_to_response(ServerError::InvalidExpire);
        }
    }

    if let Some(payload_view) = payload.max_views
        && (payload_view > max_view as i32 || payload_view < 1)
    {
        info!("Payload view is invalid. Max view is {max_view}. Gotten {payload_view}");
        return server_error_to_response(ServerError::InvalidViewCount);
    }

    db_handler
        .new_secret(payload)
        .await
        .map_or_else(server_error_to_response, |id| {
            info!("Created secret with id: {id}");
            HttpResponse::Ok().json(id)
        })
}

fn server_error_to_response(e: ServerError) -> HttpResponse {
    match e {
        ServerError::ViewAndExpireEmpty
        | ServerError::InvalidExpire
        | ServerError::InvalidViewCount => HttpResponse::BadRequest().body(e.to_string()),

        ServerError::DatabaseError(e) => {
            error!("Database error: {e}");
            HttpResponse::InternalServerError().body("internal server error")
        }
    }
}
