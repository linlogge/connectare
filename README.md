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

### 1. Define your Protocol (hello.proto)

```protobuf
syntax = "proto3";

package hello;

message HelloRequest { 
    optional string name = 1; 
}

message HelloResponse { 
    string message = 1; 
}

service HelloWorldService {
    rpc SayHello(HelloRequest) returns (HelloResponse) {}
    rpc SayHelloStream(HelloRequest) returns (stream HelloResponse) {}
}
```

### 2. Generate Code (build.rs)

```rust
use connectare_build::{connectare_codegen, ConnectareGenSettings};

fn main() {
    let settings = ConnectareGenSettings::from_directory_recursive("proto")
        .expect("failed to glob proto files");
    connectare_codegen(settings).unwrap();
}
```

### 3. Server Implementation

```rust
use axum::Router;
use connectare::prelude::*;
use axum_extra::extract::Host;
use proto::hello::*;

mod proto {
    pub mod hello {
        include!(concat!(env!("OUT_DIR"), "/hello.rs"));
    }
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        // Unary RPC handler
        .rpc(HelloWorldService::say_hello(say_hello_unary))
        // GET version for caching
        .rpc(HelloWorldService::say_hello_unary_get(say_hello_unary));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3030").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn say_hello_unary(Host(host): Host, request: HelloRequest) -> Result<HelloResponse, Error> {
    Ok(HelloResponse {
        message: format!(
            "Hello {}! You're addressing the hostname: {}.",
            request.name.unwrap_or_else(|| "unnamed".to_string()),
            host
        ),
    })
}
```

### 4. Client Implementation

```rust
use connectare::prelude::*;
use proto::hello::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = RpcClient::new(RpcClientConfig::new("http://127.0.0.1:3030")?);
    let client = HelloWorldServiceClient::new(client);

    let request = HelloRequest {
        name: Some("World".to_string()),
    };
    let response = client.say_hello(request).await?;
    println!("Response: {:?}", response);
    
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
