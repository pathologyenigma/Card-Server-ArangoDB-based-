use std::str::FromStr;

use async_graphql::{http::GraphiQLSource, EmptySubscription, Schema};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse, GraphQLSubscription};
use axum::{
    http::HeaderMap,
    response::{self, IntoResponse},
    routing::get,
    Extension, Router, Server,
};
use card_server_aragog::resolvers::{MutationRoot, QueryRoot};
use tracing::info;
type RootSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

async fn graphql_handler(
    schema: Extension<RootSchema>,
    req: GraphQLRequest,
    headers: HeaderMap,
) -> GraphQLResponse {
    let mut req = req.into_inner();
    if let Some(token) = get_token_from_headers(&headers) {
        if let Some(token) = token.0 {
            req = req.data(token);
        }
    }
    schema.execute(req).await.into()
}
async fn graphiql() -> impl IntoResponse {
    response::Html(
        GraphiQLSource::build()
            .endpoint("http://localhost:8000")
            .subscription_endpoint("ws://localhost:8000/ws")
            .finish(),
    )
}

fn get_token_from_headers(headers: &HeaderMap) -> Option<card_server_aragog::JWT> {
    headers.get("Token").and_then(|value| {
        value
            .to_str()
            .map(|s| card_server_aragog::JWT::from_str(&s.to_owned()).unwrap())
            .ok()
    })
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt().with_test_writer().init();
    let db = card_server_aragog::set_up_database().await.unwrap();
    let schema = Schema::build(
        QueryRoot::default(),
        MutationRoot::default(),
        EmptySubscription,
    )
    .data(db)
    .finish();
    let app = Router::new()
        .route("/", get(graphiql).post(graphql_handler))
        .route("/ws", GraphQLSubscription::new(schema.clone()))
        .layer(Extension(schema));
    info!("graphql IDE: http://127.0.0.1:8000");

    Server::bind(&"0.0.0.0:8000".parse().expect("failed to parse url"))
        .serve(app.into_make_service())
        .await
        .expect("failed to bind server");
}
