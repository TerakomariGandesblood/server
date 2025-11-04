use std::path::PathBuf;

use axum::Json;
use axum::body::Bytes;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum_typed_multipart::{FieldData, TryFromMultipart, TypedMultipart};
use serde::Serialize;
use tokio::fs;
use uuid::Uuid;

use crate::ServerError;

#[derive(Serialize)]
struct HealthCheck {
    ok: bool,
}

pub async fn health_check() -> impl IntoResponse {
    Json(HealthCheck { ok: true })
}

#[derive(TryFromMultipart)]
pub struct FileUpload {
    pub files: Vec<FieldData<Bytes>>,
}

pub async fn upload(
    TypedMultipart(FileUpload { files }): TypedMultipart<FileUpload>,
) -> Result<Response, ServerError> {
    if files.is_empty() {
        tracing::error!("upload file is empty");
        return Ok((StatusCode::BAD_REQUEST, "upload file is empty").into_response());
    }

    let base_path = PathBuf::from("upload");
    fs::create_dir_all(&base_path).await?;

    for file in files {
        let Some(file_name) = file.metadata.file_name else {
            tracing::error!("upload file name is empty");
            return Ok((StatusCode::BAD_REQUEST, "upload file name is empty").into_response());
        };

        let file_name = sanitize_filename::sanitize(file_name);
        let mut file_path = base_path.join(&file_name);

        if fs::try_exists(&file_path).await? {
            file_path = base_path.join(format!("{file_name}.{}", Uuid::new_v4()));
        }

        fs::write(&file_path, file.contents).await?;
        tracing::trace!("save file: {}", file_path.display());
    }

    Ok(StatusCode::CREATED.into_response())
}
