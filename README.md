


## Tide+GraphQL+Neo4j
*this project aim to testing these stacks for future production usage*

## Project Format

 - **app** for the server start up and some config things
 - the lib it self for queries mutations and subscriptions set up
 - crates/ogm-handler for a gql-client to neo4j graphql ogm server
 - crates/gql-types for types input-types and scalar-types and some serde things

## Dependencies

 - async-std: **Async version of the Rust standard library**
 - Tide.rs: a async-std based http server(check axum or warp if you need tokio based ones)
 - async-graphql(and it's tide plugin): pretty good server-side graphql lib (my personal reason for choosing this lib is that it have better and easier using scalar support)
 - surf: a async-std based  http client(check reqwest if your need tokio based ones)
 - serde(and serde-json): **a framework for  _ser_ializing and  _de_serializing Rust data structures efficiently and generically.**
 - graphql-client: a async easy-use graphql client lib with official generator cli
 - log: a logger lib
 - femme: A pretty-printer and [ndjson](http://ndjson.org/) logger for the [log](https://docs.rs/log) crate.
 - uuid: a uuid lib
 - japhonex: for checking phone numbers
 - url: for checking if url is valid
 - chorno: for date and time types
 - jsonwebtoken: serialize and deserialize jwt
 - regex: for regex checkers

