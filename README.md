# Speckle-rs

Start of a library for delving with the speckle rest API, graphQL, 3D geometry*, etc.

It is currently under development and not ready to use.

[Speckle](https://speckle.systems/) is an open source tool for all things 3D for Architecture, Engineering and Construction industry. Why not take advantage of it and create an SDK for Rust. 

Speckle-rs can be used to easily use the speckle api in Rust CLI applications. 

In the future, I plan to create a module to render 3D geometry from Speckle data using [WGPU](https://wgpu.rs/), an implementation of the WebGPU standard for Rust.

* __for now, just as an idea__

## Examples

```rs
use speckle_rs::graphql;

    let client = graphql::GQLClient::new("token");
    let request = graphql::QueryBuilder::new(stream_id, object_id)
        .where_equals("level.name", "5FL")
        .select("type")
        .build()
        .json();

    match client.send_query(request) {
        Some(res) => println!("{}", res),
        None => println!("Error"),
    }
```
