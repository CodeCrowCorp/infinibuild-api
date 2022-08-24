# infinibuild-api

The serverless API for the infinibuild project

## Contribution

This project starts you off with a `src/lib.rs` file, acting as an entrypoint for requests hitting your Worker. Feel free to add more code in this file, or create Rust modules anywhere else for this project to use.

With `wrangler`, you can build, test, and deploy your Worker with the following commands:

```sh
$ npx wrangler dev

# deploy your Worker globally to the Cloudflare network (update your wrangler.toml file for configuration)
$ npx wrangler publish
```

Read the latest `worker` crate documentation here: https://docs.rs/worker

## WebAssembly
This project compiles Rust to WebAssembly and publishes the resulting worker to Cloudflare's [edge infrastructure](https://www.cloudflare.com/network/).

`workers-rs` (the Rust SDK for Cloudflare Workers used in this template) is meant to be executed as compiled WebAssembly, and as such so **must** all the code you write and depend upon. All crates and modules used in Rust-based Workers projects have to compile to the `wasm32-unknown-unknown` triple.

Read more about this on the [`workers-rs`](https://github.com/cloudflare/workers-rs) project README.
