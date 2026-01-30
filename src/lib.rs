use std::sync::Arc;

use axum::{
    Router,
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse},
    routing,
};
use minijinja::{Environment, context, path_loader};
use minijinja_autoreload::AutoReloader;
use tower_http::services::ServeDir;

// Root handler.
async fn root(State(state): State<AppState>) -> impl IntoResponse {
    Html(state.render_template("base.html", context! {}))
}

// Return `200 OK` if the server is running.
async fn health_check() -> StatusCode {
    StatusCode::OK
}

pub fn app() -> Router {
    let serve_dir = ServeDir::new("assets");

    let reloader = AutoReloader::new(|notifier| {
        let template_path = "templates";
        let mut env = Environment::new();
        env.set_loader(path_loader(template_path));
        notifier.watch_path(template_path, true);
        Ok(env)
    });

    let app_state = AppState {
        jinja_env: Arc::new(reloader),
    };

    Router::new()
        .route("/", routing::get(root))
        .route("/health_check", routing::get(health_check))
        .nest_service("/assets", serve_dir)
        .with_state(app_state)
}

#[derive(Clone)]
struct AppState {
    jinja_env: Arc<AutoReloader>,
}

impl AppState {
    fn render_template(&self, name: &str, ctx: minijinja::Value) -> String {
        let env = self.jinja_env.acquire_env().unwrap();
        let template = env.get_template(name).unwrap();
        template.render(ctx).unwrap()
    }
}
