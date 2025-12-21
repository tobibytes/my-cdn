pub mod media;
pub fn media_service_init() -> media::MediaService {
    media::MediaService::new()
}