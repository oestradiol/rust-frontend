#!forbid(unsafe_code)

use actix_web::{dev::Service as _, HttpServer, App, Error, middleware::Logger, get, HttpResponse, HttpRequest};
use actix_files as actix_fs;
use actix_web::web::Data;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use web::{ServerApp, ServerAppProps};
use futures::FutureExt;

pub struct AppState {
  pub index_html_before: String,
  pub index_html_after: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  // Logging
  std::env::set_var("RUST_LOG", "debug");
  std::env::set_var("RUST_BACKTRACE", "1");
  std::env::set_var("RUST_LIB_BACKTRACE", "full");
  tracing_subscriber::fmt::init();
  color_eyre::install().unwrap_or_default();

  // Static HTML String
  let index_html_s = tokio::fs::read_to_string("./dist/index.html")
    .await.expect("failed to read index.html");
  let (index_html_before, index_html_after) = index_html_s.split_once("</body>").unwrap();
  let mut index_html_after = index_html_after.to_owned();
  index_html_after.insert_str(0, "</body>");
  let index_html_before = index_html_before.to_owned();

  // Configure Origin
  let sock_addr = SocketAddr::from((
    IpAddr::V4(Ipv4Addr::LOCALHOST),
    8080,
  ));

  // Configure Server
  let server = HttpServer::new(move || {
    App::new()
      // Static App Data
      .app_data(Data::new(AppState {
        index_html_after: index_html_after.clone(),
        index_html_before: index_html_before.clone()
      }))

      // Logger
      .wrap(Logger::default())

      // Middleware
      .wrap_fn(|req, srv| {
          // Before any response
          srv.call(req).map(|res| {
              // Immediately before response
              res
          })
      })

      //// Everything else goes here!

      // Configure Static Files
      .service(
        actix_fs::Files::new("/dist", "./dist")
      )

      // Configure Yew App's Renderer Service/Route
      .service(render_yew_app)
  })
  .bind(sock_addr)?
  .run();

  tracing::info!("listening on http://{}", sock_addr);

  server.await
}

#[get("/{tail:.*}")]
async fn render_yew_app(data: Data<AppState>, req: HttpRequest) -> Result<HttpResponse, Error> {
  let url = req.uri().to_string();
  let renderer = yew::ServerRenderer::<ServerApp>::with_props(move || ServerAppProps {
    url: url.into()
  }).render().await;

  let body = data.index_html_before.clone() + &*renderer + &*data.index_html_after.clone();

  Result::<HttpResponse, Error>::Ok(HttpResponse::Ok().body(body))
}

#[cfg(unix)]
#[global_allocator]
static GLOBAL: jemallocator::Jemalloc = jemallocator::Jemalloc;
