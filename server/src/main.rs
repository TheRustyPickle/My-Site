mod auth;
mod endpoints;
mod ws;

use actix_cors::Cors;
use actix_files::Files;
use actix_web::dev;
use actix_web::web::{Data, Json, Path};
use actix_web::{App, Error, HttpRequest, HttpResponse, HttpServer, http, web};
use app::App;
use chrono::{Days, Utc};
use dashmap::DashMap;
use db::{get_connection as get_db_connection, get_redis_connection};
use dev::Service;
use leptos::config::get_configuration;
use leptos::prelude::*;
use leptos_actix::{LeptosRoutes, generate_route_list};
use leptos_meta::MetaTags;
use log::{LevelFilter, error, info};
use reqwest::Client;
use std::env::var;
use std::sync::{Arc, OnceLock};
use std::time::Duration;
use tokio::task::{spawn, spawn_local};
use tokio::time::sleep;
use vial_shared::CreateSecretRequest;
use vial_shared::config::Config;
use vial_srv::db::{Handler, get_connection};
use vial_srv::errors::ServerError;
use web::{Payload, resource};

use crate::auth::{clean_up_verifier_code, discord_callback};
use crate::endpoints::{task_redirect, upload_avatar};
use crate::ws::server::{Server, ServerInterface, handler};

pub static JWT_SECRET: OnceLock<String> = OnceLock::new();
pub static REDIS_URL: OnceLock<String> = OnceLock::new();

pub static IMAGEKIT_PUBLIC: OnceLock<String> = OnceLock::new();
pub static IMAGEKIT_PRIVATE: OnceLock<String> = OnceLock::new();
pub static IMAGEKIT_URL: OnceLock<String> = OnceLock::new();

pub static DISCORD_CLIENT_ID: OnceLock<String> = OnceLock::new();
pub static DISCORD_CLIENT_SECRET: OnceLock<String> = OnceLock::new();
pub static DISCORD_REDIRECT_URI: OnceLock<String> = OnceLock::new();
pub static DISCORD_REDIRECT_FULL: OnceLock<String> = OnceLock::new();
pub static DISCORD_TOKEN: OnceLock<String> = OnceLock::new();

pub static TELEGRAM_REDIRECT: OnceLock<String> = OnceLock::new();
pub static TELEGRAM_TOKEN: OnceLock<String> = OnceLock::new();

pub static BACKEND_URL: OnceLock<String> = OnceLock::new();

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();

    pretty_env_logger::formatted_timed_builder()
        .format_timestamp_millis()
        .filter_level(LevelFilter::Info)
        .filter_module("tracing::span", LevelFilter::Off)
        .filter_module("serenity", LevelFilter::Off)
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

    let database_url_tbd = var("DATABASE_URL_TBD").expect("DATABASE_URL_TBD must be set");

    let jwt_secret = var("JWT_SECRET").expect("JWT_SECRET must be set");

    let redis_url = var("REDIS_URL").expect("REDIS_URL must be set");

    let imagekit_public = var("IMAGEKIT_PUBLIC").expect("IMAGEKIT_PUBLIC must be set");

    let imagekit_private = var("IMAGEKIT_PRIVATE").expect("IMAGEKIT_PRIVATE must be set");

    let imagekit_url = var("IMAGEKIT_URL").expect("IMAGEKIT_URL must be set");

    let discord_client_id = var("DISCORD_CLIENT_ID").expect("DISCORD_CLIENT_ID must be set");

    let discord_client_secret =
        var("DISCORD_CLIENT_SECRET").expect("DISCORD_CLIENT_SECRET must be set");

    let discord_redirect_full = var("DISCORD_REDIRECT_FULL").expect("DISCORD_REDIRECT must be set");

    let discord_redirect_uri = var("DISCORD_REDIRECT_URI").expect("DISCORD_REDIRECT must be set");

    let telegram_redirect = var("TELEGRAM_REDIRECT").expect("TELEGRAM_REDIRECT must be set");

    let telegram_token = var("TELEGRAM_TOKEN").expect("TELEGRAM_TOKEN must be set");

    let discord_token = var("DISCORD_TOKEN").expect("DISCORD_TOKEN must be set");

    let backend_url = var("BACKEND_URL").expect("BACKEND_URL must be set");

    JWT_SECRET
        .set(jwt_secret)
        .expect("JWT_SECRET must be set only once");

    REDIS_URL
        .set(redis_url.clone())
        .expect("REDIS_URL must be set only once");

    IMAGEKIT_PUBLIC
        .set(imagekit_public)
        .expect("IMAGEKIT_PUBLIC must be set only once");

    IMAGEKIT_PRIVATE
        .set(imagekit_private)
        .expect("IMAGEKIT_PRIVATE must be set only once");

    IMAGEKIT_URL
        .set(imagekit_url)
        .expect("IMAGEKIT_URL must be set only once");

    DISCORD_CLIENT_ID
        .set(discord_client_id)
        .expect("DISCORD_CLIENT_ID must be set only once");

    DISCORD_CLIENT_SECRET
        .set(discord_client_secret)
        .expect("DISCORD_CLIENT_SECRET must be set only once");

    DISCORD_REDIRECT_FULL
        .set(discord_redirect_full)
        .expect("DISCORD_REDIRECT_FULL must be set only once");

    DISCORD_REDIRECT_URI
        .set(discord_redirect_uri)
        .expect("DISCORD_REDIRECT_URI must be set only once");

    TELEGRAM_REDIRECT
        .set(telegram_redirect)
        .expect("TELEGRAM_REDIRECT must be set only once");

    TELEGRAM_TOKEN
        .set(telegram_token)
        .expect("TELEGRAM_TOKEN must be set only once");

    BACKEND_URL
        .set(backend_url)
        .expect("BACKEND_URL must be set only once");

    DISCORD_TOKEN
        .set(discord_token)
        .expect("DISCORD_TOKEN must be set only once");

    let verifier_list = Arc::new(DashMap::new());

    spawn(clean_up_verifier_code(verifier_list.clone()));

    let pool = get_db_connection(&database_url_tbd).await;
    let redis_conn = get_redis_connection(&redis_url).await;

    let (server, handler, cmd_rx) =
        Server::new(pool.clone(), redis_conn.clone(), verifier_list.clone());

    let server_clone = server.clone();
    spawn(server.run(cmd_rx));

    // TODO: Fix cors url

    HttpServer::new(move || {
        let server_clone = server_clone.clone();
        let handler_clone = handler.clone();
        let verifier_list = verifier_list.clone();
        let pool = pool.clone();
        let redis_conn = redis_conn.clone();

        // Generate the list of routes in your Leptos App
        let routes = generate_route_list(App);
        let leptos_options = &conf.leptos_options;
        let site_root = leptos_options.site_root.clone().to_string();

        let cors_conf = Cors::default()
            .allowed_origin_fn(|origin, _req_head| {
                matches!(
                    origin.to_str(),
                    Ok("http://localhost:3000"
                        | "https://origil.netlify.app"
                        | "http://127.0.0.1:3000")
                )
            })
            .allowed_methods(vec!["GET", "POST", "DELETE", "OPTIONS"])
            .allowed_headers(vec![
                http::header::AUTHORIZATION,
                http::header::CONTENT_TYPE,
            ])
            .supports_credentials()
            .max_age(3600);

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
            .app_data(Data::new(handler_clone))
            .app_data(Data::new(server_clone))
            .app_data(Data::new(verifier_list))
            .app_data(Data::new(pool))
            .app_data(Data::new(redis_conn))
            .service(resource("/ws").route(web::get().to(start_ws)))
            .service(resource("/auth/discord").route(web::get().to(discord_callback)))
            .service(resource("/redirect").route(web::get().to(task_redirect)))
            .service(
                web::scope("/upload-avatar")
                    .wrap(cors_conf)
                    .route("", web::post().to(upload_avatar)),
            )
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

#[derive(Clone, Debug)]
pub struct UserIpAgent {
    pub ip: String,
    pub user_agent: String,
}

async fn start_ws(
    req: HttpRequest,
    stream: Payload,
    handler: Data<ServerInterface>,
) -> Result<HttpResponse, Error> {
    let (response, session, msg_stream) = actix_ws::handle(&req, stream)?;

    let ip_local = req
        .peer_addr()
        .map_or_else(|| "unknown".to_string(), |addr| addr.ip().to_string());

    let user_agent = req
        .headers()
        .get("user-agent")
        .map(|v| v.to_str().unwrap_or_default())
        .unwrap_or_default()
        .to_string();

    let forwarded_ip = req
        .headers()
        .get("X-Forwarded-For")
        .map(|v| v.to_str().unwrap_or_default())
        .unwrap_or_default()
        .to_string();

    let ip = if forwarded_ip.is_empty() {
        ip_local
    } else {
        forwarded_ip
    };

    let user_ip_agent = UserIpAgent { ip, user_agent };

    spawn_local(handler::handle_ws(
        (**handler).clone(),
        session,
        msg_stream,
        user_ip_agent,
    ));
    Ok(response)
}
