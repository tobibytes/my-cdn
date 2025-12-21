use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use crate::{models::{UploadCompleteBody, UploadInitBody}, services::{media::MediaService, media_service_init}};

#[derive(Clone)]
pub struct MediaController {
    service: MediaService
}

impl MediaController {
    pub fn new() -> MediaController{
        let ms = media_service_init();
        MediaController {
            service: ms
        }
    }

    pub async fn upload_init(&self, body: UploadInitBody) -> Response {
        match self.service.upload_init(body).await {
            Ok(res) => Json(res).into_response(),
            Err(_) => (StatusCode::BAD_REQUEST).into_response(),
        }
    }

    pub async fn upload_complete(&self, body: UploadCompleteBody) -> Response {
        match self.service.upload_complete(body).await {
            Ok(res) => Json(res).into_response(),
            Err(_) => (StatusCode::BAD_REQUEST).into_response(),
        }
    }
}

pub async fn upload_init_handler(
    State(controller): State<MediaController>,
    Json(body): Json<UploadInitBody>,
) -> Response {
    controller.upload_init(body).await
}

pub async fn upload_complete_handler(
    State(controller): State<MediaController>,
    Json(body): Json<UploadCompleteBody>,
) -> Response {
    controller.upload_complete(body).await
}
