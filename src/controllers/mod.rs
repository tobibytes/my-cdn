pub mod media;

pub fn media_controller_init() -> media::MediaController {
    media::MediaController::new()
}