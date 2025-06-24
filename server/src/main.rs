use actix_files::Files;
use actix_web::*;
use app::App;
use dev::Service;
use leptos::config::get_configuration;
use leptos::prelude::*;
use leptos_actix::{generate_route_list, LeptosRoutes};
use leptos_meta::MetaTags;
use log::{info, LevelFilter};
use reqwest::Client;
use std::env::var;
use std::time::Duration;
use tokio::time::sleep;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();

    pretty_env_logger::formatted_timed_builder()
        .format_timestamp_millis()
        .filter_level(LevelFilter::Info)
        .init();

    let conf = get_configuration(None).unwrap();

    let port = var("PORT")
        .ok()
        .and_then(|p| p.parse::<u16>().ok())
        .unwrap_or(8080);

    let address = var("ADDRESS").unwrap_or("0.0.0.0".to_string());

    let addr = format!("{address}:{port}");
    let addr_clone = addr.clone();

    tokio::spawn(ping_site());

    HttpServer::new(move || {
        // Generate the list of routes in your Leptos App
        let routes = generate_route_list(App);
        let leptos_options = &conf.leptos_options;
        let site_root = leptos_options.site_root.clone().to_string();

        info!("listening on http://{}", addr_clone);

        App::new()
            .wrap_fn(|req, srv| {
                let path = req.path();

                let ip = req
                    .connection_info()
                    .realip_remote_addr()
                    .unwrap_or("Unknown")
                    .to_string();

                if !path.starts_with("/pkg/")
                    && !path.ends_with(".js")
                    && !path.ends_with(".js")
                    && !path.ends_with(".css")
                    && !path.ends_with(".wasm")
                    && !path.starts_with("/assets/")
                    && !path.starts_with("/favicon.ico")
                    && path != "/sw.js"
                    && &ip != "54.254.162.138"
                {
                    info!("Serving data for path: {path}. Request gotten from: {ip}",);
                }

                let fut = srv.call(req);
                async {
                    let res = fut.await?;
                    Ok(res)
                }
            })
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
        sleep(Duration::from_secs(850)).await;
    }
}
