# my-cdn

A small CDN API for direct-to-S3 uploads, written in Rust with `axum`. Hands out short-lived presigned `PUT` URLs so files travel browser-to-S3 (the API server never proxies bytes), then records the result.

Live: **`https://api.cdn.tobiolajide.com`**

## Why

When an app needs to accept user file uploads, two patterns are common:

1. Stream the upload through your server. Simple, but you pay the bandwidth and your process holds the request open.
2. Hand the client a presigned S3 URL and let it upload directly. Fast, cheap, your server stays free.

This is (2), packaged as a reusable service so any of my apps can drop in `POST /upload/init` and get a working upload pipeline.

## API

### `POST /upload/init`

Request a presigned upload URL.

```json
{
  "filename":     "image.png",
  "content_type": "image/png",
  "size":         12345,
  "app":          "my-app"
}
```

Response:

```json
{
  "upload_url": "https://...s3.amazonaws.com/...?X-Amz-Signature=...",
  "key":        "my-app/<uuid>-image.png",
  "public_url": "https://cdn.example.com/my-app/<uuid>-image.png"
}
```

### `PUT <upload_url>`

Browser uploads the file body directly to S3 with `Content-Type: <content_type>`.

### `POST /upload/complete`

Notify the API once the upload finishes:

```json
{ "key": "my-app/<uuid>-image.png", "status": "success" }
```

See [`API.md`](./API.md) for a copy-pasteable JS client example.

## Architecture

```
Browser ──POST /upload/init──▶ axum (this repo) ──▶ AWS SDK (presign)
   │                                  │
   └──── PUT presigned URL ──▶ S3 ◀───┘
                                      │
   ◀──── POST /upload/complete ──────┘
```

- `controllers/` — request handlers
- `services/media.rs` — S3 client + presigned URL generation
- `models/` — request/response DTOs

## Run it

### Local

```sh
export PORT=8000
export FRONTEND_URL=http://localhost:3000
# plus AWS credentials + bucket config

cargo run
```

### Docker

```sh
docker build -t my-cdn .
docker run --env-file .env -p 8000:8000 my-cdn
```

The included `Dockerfile` is multi-stage (builder + slim `debian:bookworm-slim` runtime) and runs the binary as a non-root `appuser`.

## Stack

Rust 2024 · `axum` 0.8 · `tokio` · `tower-http` (CORS, tracing) · AWS SDK for Rust (`aws-sdk-s3`) · `tracing` / `tracing-subscriber` · multi-stage `Dockerfile`
