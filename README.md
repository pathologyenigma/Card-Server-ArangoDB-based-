## Card-Server(ArangoDB based)
*card server using arangodb as the database*

## Project Format

 - **app** for the server start up and some config things
 - the lib it self for queries mutations and subscriptions set up
 - crates/ogm for aragog ogm models and services
 - crates/gql-types for types input-types and scalar-types and some serde things

## Dependencies

 - tokio.rs: **probably the most popular and common async runtime in rust**
 - Axum.rs: a http server lib based on hyper and writen by the tokio.rs team
 - async-graphql(and it's tide plugin): pretty good server-side graphql lib (my personal reason for choosing this lib is that it have better and easier using scalar support)
 - aragog: a arangodb ogm
 - serde(and serde-json): **a framework for  _ser_ializing and  _de_serializing Rust data structures efficiently and generically.**
 - tracing and tracing-subscriber:  for logger usage
 - japhonex: for checking phone numbers
 - url: for checking if url is valid
 - chorno: for date and time types
 - jsonwebtoken: serialize and deserialize jwt
 - regex: for regex checkers
 - dotenv: for read .env file