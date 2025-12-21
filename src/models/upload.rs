
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct UploadInitBody {
    pub filename: String,
    pub size: usize,
    pub content_type: String,
    pub app: String,
}

#[derive(Serialize)]
pub struct UploadInitResponse {
    pub upload_url: String,
    pub key: String,
    pub public_url: String,
}

#[derive(Deserialize)]
pub struct UploadCompleteBody {
    pub key: String,
    pub status: String,
}

#[derive(Serialize)]
pub struct UploadCompleteResponse {
    pub message: String,
}
