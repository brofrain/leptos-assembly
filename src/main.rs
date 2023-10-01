use leptos_assembly::{logger, App};
use axum::{
    body::{boxed, Body, BoxBody},
    extract::State,
    http::{Request, Response, Uri},
    response::{IntoResponse, Response as AxumResponse},
    routing::post,
    Router,
};
use leptos::{get_configuration, view, LeptosOptions, View};
use leptos_axum::{generate_route_list, LeptosRoutes};
use tower::ServiceExt;
use tower_http::services::ServeDir;

fn client_app() -> View {
    view! { <App/> }
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

async fn file_and_error_handler(
    uri: Uri,
    State(options): State<LeptosOptions>,
    req: Request<Body>,
) -> AxumResponse {
    let root = &options.site_root;
    let response = get_static_file(root, &uri).await;

    if response.status().is_success() {
        response.into_response()
    } else {
        let handler =
            leptos_axum::render_app_to_stream(options.clone(), client_app);
        handler(req).await.into_response()
    }
}

#[tokio::main]
async fn main() {
    logger::init();

    let conf = get_configuration(None).await.unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(client_app);

    let app = Router::new()
        .route("/api/*fn_name", post(leptos_axum::handle_server_fns))
        .leptos_routes(&leptos_options, routes, client_app)
        .fallback(file_and_error_handler)
        .with_state(leptos_options);

    log::info!("Listening on http://{}", &addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
