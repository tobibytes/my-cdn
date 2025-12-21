# CDN API Guide

Base URL: `http://localhost:8000` (adjust to your deployment).

All requests use JSON. Set `Content-Type: application/json`.

## 1) Request an upload URL
`POST /upload/init`

Body:
```json
{
  "filename": "image.png",
  "content_type": "image/png",
  "size": 12345,
  "app": "my-app"
}
```

Response:
```json
{
  "upload_url": "<presigned PUT url>",
  "key": "uploads/my-app/<uuid>-image.png",
  "public_url": "https://<account>.r2.cloudflarestorage.com/<bucket>/uploads/my-app/<uuid>-image.png"
}
```

Notes:
- `key` is generated server-side (UUID prefix); use it for future references.
- Use the returned `upload_url` exactly as-is for the next step.

## 2) Upload the file to R2
`PUT <upload_url>`

Headers:
- `Content-Type: <same content_type you sent in step 1>`

Body:
- Raw file bytes.

If the request succeeds (HTTP 200/204), the object is stored at `key`.

## 3) Report completion status
`POST /upload/complete`

Body:
```json
{
  "key": "uploads/my-app/<uuid>-image.png",
  "status": "success"
}
```

Response:
```json
{ "message": "Upload success for uploads/my-app/<uuid>-image.png" }
```

Use `status` values like `"success"` or `"failed"` to indicate the outcome of your client-side upload attempt.

## Minimal curl example
```sh
# Step 1: get presigned URL
INIT_RES=$(curl -s -X POST http://localhost:8000/upload/init \
  -H "Content-Type: application/json" \
  -d '{"filename":"image.png","content_type":"image/png","size":123,"app":"my-app"}')
UPLOAD_URL=$(echo "$INIT_RES" | jq -r .upload_url)
KEY=$(echo "$INIT_RES" | jq -r .key)

# Step 2: upload file
curl -X PUT "$UPLOAD_URL" -H "Content-Type: image/png" --data-binary @image.png

# Step 3: report completion
curl -X POST http://localhost:8000/upload/complete \
  -H "Content-Type: application/json" \
  -d "{\"key\":\"$KEY\",\"status\":\"success\"}"
```

## CORS / browsers
- The presigned `upload_url` is suitable for direct browser `fetch`/`PUT`.
- Ensure your R2 bucket CORS allows your frontend origin, method `PUT`, and header `Content-Type`.
