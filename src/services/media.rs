use std::time::Duration;

use aws_config::Region;
use aws_credential_types::Credentials;
use aws_sdk_s3::{config::Builder as S3ConfigBuilder, presigning::PresigningConfig, Client};
use anyhow::Result;
use uuid::Uuid;
use tracing::{info, warn};

use crate::models::{UploadCompleteBody, UploadCompleteResponse, UploadInitBody, UploadInitResponse};

#[derive(Clone)]
pub struct MediaService {
    bucket: String,
    endpoint_url: String,
    pub_url: String,
    client: Client,
}

fn load_env_first(keys: &[&str], label: &str) -> String {
    for key in keys {
        match std::env::var(key) {
            Ok(val) => {
                if val.is_empty() {
                    warn!(env_key=%key, "{} is set but empty", label);
                } else {
                    info!(env_key=%key, "using {} from env", label);
                }
                return val;
            }
            Err(_) => warn!(env_key=%key, "env var for {} not set", label),
        }
    }
    String::new()
}

fn normalize_endpoint(account_id: &str) -> String {
    format!("https://{}.r2.cloudflarestorage.com", account_id)
}

impl MediaService {
    pub fn new() -> MediaService {
        let ac_key = load_env_first(&["R2_ACCESS_KEY", "R2_ACCOUNT_ACCESS_KEY"], "R2 access key");
        let ac_id = load_env_first(&["R2_ACCOUNT_ID"], "R2 account id");
        let bucket = load_env_first(&["R2_BUCKET"], "R2 bucket");
        let s_key = load_env_first(&["R2_SECRET_KEY", "R2_ACCOUNT_SECRET_KEY"], "R2 secret key");
        let pub_url = load_env_first(&["R2_PUBLIC_URL"], "R2 public url");
        let endpoint_url = normalize_endpoint(&ac_id);
        let creds = Credentials::new(ac_key, s_key, None, None, "r2");
        let region = Region::new("auto");
        let s3_config = S3ConfigBuilder::new()
            .behavior_version_latest()
            .region(region)
            .endpoint_url(endpoint_url.clone())
            .force_path_style(true)
            .credentials_provider(creds)
            .build();
        let client = Client::from_conf(s3_config);
        info!(bucket=%bucket, "media service configured");
        MediaService { client, bucket, endpoint_url, pub_url }
    }


    pub async fn upload_init(&self, body: UploadInitBody) -> Result<UploadInitResponse> {
        let presign_cfg = PresigningConfig::expires_in(Duration::from_secs(600))?;

        let UploadInitBody { filename, size, content_type, app } = body;
        let app_folder = if app.trim().is_empty() { "default".to_string() } else { app.trim().to_string() };
        info!(%filename, %size, %content_type, app=%app_folder, bucket=%self.bucket, "generating presigned upload URL");
        let key = format!("{}/{}-{}", app_folder, Uuid::new_v4(), filename);
        let presigned = self.client
            .put_object()
            .bucket(&self.bucket)
            .key(&key)
            .content_type(content_type)
            .presigned(presign_cfg)
            .await?;
        
        let uri = presigned.uri().to_string();
        let public_url = format!("{}/{}", self.pub_url.trim_end_matches('/'), key);
        info!(%uri, %key, %public_url, "presigned URL generated");
        Ok(UploadInitResponse {
            upload_url: uri,
            key,
            public_url,
        })
    }

    pub async fn upload_complete(&self, body: UploadCompleteBody) -> Result<UploadCompleteResponse> {
        let UploadCompleteBody { key, status } = body;
        info!(%key, %status, "upload completion reported");
        Ok(UploadCompleteResponse {
            message: format!("Upload {status} for {key}"),
        })
    }

}
