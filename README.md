# Connectare

Connect-Web RPC for Rust

## Overview

Connectare is a Rust library that provides Connect-Web RPC functionality for Axum applications. This project combines and extends concepts from two excellent upstream projects:

- **[axum-connect](https://github.com/AThilenius/axum-connect/)** - Server-side Connect-Web implementation for Axum
- **[connect-rpc-rs](https://github.com/lann/connect-rpc-rs)** - Connect RPC client implementation

## Features

- **Axum Integration**: Seamlessly integrates with existing Axum applications
- **Type Safety**: Strongly typed request/response handling
- **Streaming Support**: Both unary and server-streaming RPC methods
- **Protocol Compliance**: Full Connect-Web protocol support
- **Macro Generateion**: Generate client & server from Protobuf files

## Quick Start

### Server

```rust
use connectare::prelude::*;
use axum::{Router, extract::Host};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .rpc(HelloWorldService::say_hello(say_hello_handler));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3030").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn say_hello_handler(Host(host): Host, request: HelloRequest) -> Result<HelloResponse, Error> {
    Ok(HelloResponse {
        message: format!("Hello {}! Host: {}", request.name, host),
    })
}
```

### Client

```rust
use connectare::client::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = ConnectClient::new("http://localhost:3030");
    let response = client.call_unary(HelloWorldService::say_hello(), HelloRequest {
        name: "World".to_string(),
    }).await?;
    
    println!("Response: {}", response.message);
    Ok(())
}
```

## Attribution

This project contains code derived from the following open source projects:

### axum-connect
- **Repository**: https://github.com/AThilenius/axum-connect/
- **Author**: Alec Thilenius
- **License**: Apache License 2.0 and MIT License
- **Original Work**: Server-side Connect-Web implementation for Axum

### connect-rpc-rs  
- **Repository**: https://github.com/lann/connect-rpc-rs
- **Author**: Lann
- **License**: Apache License 2.0
- **Original Work**: Connect RPC client implementation

All original copyright notices and license terms from the upstream projects are preserved and apply to the respective portions of code derived from those projects.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
