use axum::{
    body::{boxed, Body, BoxBody},
    extract::State,
    http::{Request, Response, Uri},
    response::{Html, IntoResponse},
    routing::{get, post},
    Router,
};
use leptos::{get_configuration, IntoView, LeptosOptions};
use leptos_axum::{generate_route_list_with_exclusions, LeptosRoutes};
use leptos_integration_utils::html_parts_separated;
use tower::ServiceExt;
use tower_http::{services::ServeDir, trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

async fn pwa_index_handler(
    State(options): State<LeptosOptions>,
) -> Html<String> {
    let (head, tail) = html_parts_separated(&options, None);
    Html(format!("{head}</head><body>{tail}"))
}

async fn get_static_file(root: &str, uri: &Uri) -> Response<BoxBody> {
    let request = Request::builder()
        .uri(uri.clone())
        .body(Body::empty())
        .unwrap();

    ServeDir::new(root)
        .oneshot(request)
        .await
        .unwrap()
        .map(boxed)
}

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
            "warn",
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

    let file_and_error_handler =
        move |uri: Uri,
              State(options): State<LeptosOptions>,
              req: Request<Body>| async move {
            let root = &options.site_root;
            let response = get_static_file(root, &uri).await;

            if response.status().is_success() {
                response.into_response()
            } else {
                let handler =
                    leptos_axum::render_app_to_stream(options.clone(), app);
                handler(req).await.into_response()
            }
        };

    let router = Router::new()
        .route("/api/*fn_name", post(leptos_axum::handle_server_fns))
        .route("/pwa", get(pwa_index_handler))
        .leptos_routes(&leptos_options, routes, app)
        .fallback(file_and_error_handler)
        .with_state(leptos_options)
        .layer(TraceLayer::new_for_http());

    log::info!("Listening on http://{addr}");

    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .unwrap();
}
