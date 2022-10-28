use async_graphql::http::GraphiQLSource;
use async_graphql::{EmptySubscription, Schema};
use tide::{http::mime, log::info, Body, Response, StatusCode};

fn main() -> async_graphql::Result<()> {
    femme::start();
    async_std::task::block_on(run())
}
async fn run() -> async_graphql::Result<()> {
    let schema = Schema::build(
        prms3::query::QueryRoot::default(),
        prms3::mutation::MutationRoot::default(),
        EmptySubscription,
    )
    .finish();
    info!("GraphiQL IDE: http://localhost:8000");
    let mut app = tide::new();
    app.at("/graphql")
        .post(async_graphql_tide::graphql(schema.clone()));

    app.at("/").get(|_| async move {
        let mut resp = Response::new(StatusCode::Ok);
        resp.set_body(Body::from_string(
            GraphiQLSource::build()
                .endpoint("http://localhost:8000/graphql")
                .subscription_endpoint("ws://localhost:8080/graphql")
                .finish(),
        ));
        resp.set_content_type(mime::HTML);
        Ok(resp)
    });

    app.listen("127.0.0.1:8000").await?;
    Ok(())
}
