#![feature(async_closure)]
#![recursion_limit = "256"]

use {
    actix::Actor,
    actix_web::{App, HttpServer},
};

pub mod errors;

macro_rules! featured {
    ($app: ident, $feature: expr, $connect: expr) => {
        #[cfg(feature = $feature)]
        let $app = $connect;
    };
}

#[actix_rt::main]
async fn main() -> Result<(), String> {
    dotenv::dotenv().ok();
    pretty_env_logger::init();

    let web_config = jirs_config::web::Configuration::read();

    let db_addr = actix::SyncArbiter::start(
        jirs_config::database::Configuration::read().concurrency,
        database_actor::DbExecutor::default,
    );
    let mail_addr = actix::SyncArbiter::start(
        jirs_config::mail::Configuration::read().concurrency,
        mail_actor::MailExecutor::default,
    );
    let hi_addr = actix::SyncArbiter::start(
        jirs_config::hi::Configuration::read().concurrency,
        highlight_actor::HighlightActor::default,
    );
    #[cfg(feature = "local-storage")]
    let fs_addr = actix::SyncArbiter::start(
        jirs_config::fs::Configuration::read().concurrency,
        filesystem_actor::FileSystemExecutor::default,
    );
    #[cfg(feature = "aws-s3")]
    let amazon_addr = actix::SyncArbiter::start(
        jirs_config::web::Configuration::read().concurrency,
        amazon_actor::AmazonExecutor::default,
    );

    let ws_server = websocket_actor::server::WsServer::start_default();

    HttpServer::new(move || {
        let app = App::new().wrap(actix_web::middleware::Logger::default());

        // data step
        let app = app
            .data(ws_server.clone())
            .data(db_addr.clone())
            .data(mail_addr.clone())
            .data(hi_addr.clone())
            .data(database_actor::build_pool());
        featured! { app, "local-storage", app.data(fs_addr.clone()) };
        featured! { app, "aws-s3", app.data(amazon_addr.clone()) };

        // services step
        let app = app
            .service(websocket_actor::index)
            .service(actix_web::web::scope("/avatar").service(web_actor::avatar::upload));

        featured! { app, "local-storage", app.service(filesystem_actor::service()) };
        app
    })
    .workers(web_config.concurrency)
    .bind(web_config.addr())
    .map_err(|e| format!("{}", e))?
    .run()
    .await
    .expect("Server internal error");
    Ok(())
}
