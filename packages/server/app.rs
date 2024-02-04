use axum::{
    body::Body,
    extract::State,
    http::{Request, StatusCode, Uri},
    response::{Html, IntoResponse, Response},
    routing::{get, post},
    Router,
};
use leptos::{get_configuration, IntoView, LeptosOptions};
use leptos_axum::{generate_route_list_with_exclusions, LeptosRoutes};
use leptos_integration_utils::html_parts_separated;
use tokio::net::TcpListener;
use tower::ServiceExt;
use tower_http::{services::ServeDir, trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

async fn pwa_app_handler(State(options): State<LeptosOptions>) -> Html<String> {
    let (head, tail) = html_parts_separated(&options, None);
    Html(format!("{head}</head><body>{tail}"))
}

async fn get_static_file(root: &str, uri: &Uri) -> Response {
    let req = Request::builder()
        .uri(uri.clone())
        .body(Body::empty())
        .unwrap();

    match ServeDir::new(root).oneshot(req).await {
        Ok(res) => res.into_response(),
        Err(err) => {
            log::error!("Failed to serve a static file: {err:?}");
            Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::empty())
                .unwrap()
        }
    }
}

async fn heathcheck() {}

pub async fn serve_app<App, AppView>(app: App)
where
    App: Fn() -> AppView + Clone + Copy + Send + Sync + 'static,
    AppView: IntoView + 'static,
{
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::from(
            #[cfg(debug_assertions)]
            "debug,hyper=info",
            #[cfg(not(debug_assertions))]
            "info",
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let conf = get_configuration(None).await.unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list_with_exclusions(
        app,
        Some(vec!["/*path".to_owned()]),
    );

    let app_handler =
        leptos_axum::render_app_to_stream(leptos_options.clone(), app);

    let file_and_error_handler =
        move |uri: Uri,
              State(options): State<LeptosOptions>,
              req: Request<Body>| async move {
            let root = &options.site_root;
            let res = get_static_file(root, &uri).await;

            if res.status() == StatusCode::NOT_FOUND {
                app_handler(req).await.into_response()
            } else {
                res
            }
        };

    let router = Router::new()
        .route("/health", get(heathcheck))
        .route("/pwa", get(pwa_app_handler))
        .route("/api/*fn_name", post(leptos_axum::handle_server_fns))
        .leptos_routes(&leptos_options, routes, app)
        .fallback(file_and_error_handler)
        .with_state(leptos_options)
        .layer(TraceLayer::new_for_http());

    let listener = TcpListener::bind(&addr).await.unwrap();

    log::info!("Listening on http://{addr}");

    axum::serve(listener, router.into_make_service())
        .await
        .unwrap();
}
