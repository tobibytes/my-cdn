# CDN API (quick guide)

Base URL: `https://api.cdn.tobiolajide.com`

All endpoints use JSON.

## 1) Get a presigned upload
`POST /upload/init`

Body:
```json
{ "filename": "image.png", "content_type": "image/png", "size": 12345, "app": "my-app" }
```

Response:
```json
{ "upload_url": "<PUT url>", "key": "my-app/<uuid>-image.png", "public_url": "https://..." }
```

## 2) Upload the file
`PUT <upload_url>` with header `Content-Type: <your content_type>` and the raw file as the body.

## 3) Tell the API the result
`POST /upload/complete` with:
```json
{ "key": "uploads/my-app/<uuid>-image.png", "status": "success" }
```

## Minimal JavaScript example
```js
async function uploadFile(file, app = "my-app") {
  const initRes = await fetch("https://api.cdn.tobiolajide.com/upload/init", {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({
      filename: file.name,
      content_type: file.type,
      size: file.size,
      app,
    }),
  }).then(r => r.json());

  await fetch(initRes.upload_url, {
    method: "PUT",
    headers: { "Content-Type": file.type },
    body: file,
  });

  await fetch("https://api.cdn.tobiolajide.com/upload/complete", {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({ key: initRes.key, status: "success" }),
  });

  console.log("Public URL:", initRes.public_url);
}
```
