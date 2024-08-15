use actix_multipart::Multipart;
use actix_web::HttpResponse;
use futures_util::StreamExt;
use tokio::io::AsyncWriteExt;

use super::standard_response::StandardResponse;

pub async fn save_file(
    mut payload: Multipart,
    file_path: std::path::PathBuf,
) -> Result<HttpResponse, actix_web::error::Error> {
    let mut file = tokio::fs::File::create(file_path).await?;

    while let Some(field) = payload.next().await {
        let mut field = match field {
            Ok(field) => field,
            Err(e) => {
                return Err(actix_web::error::ErrorBadRequest(format!(
                    "Extract File from Multipart Error: {}",
                    e.to_string()
                )))
            }
        };

        if field.name() == Some("file") {
            while let Some(chunk) = field.next().await {
                let chunk = match chunk {
                    Ok(chunk) => chunk,
                    Err(e) => {
                        return Err(actix_web::error::ErrorBadRequest(format!(
                            "Retrieve Byte Data Error: {}",
                            e.to_string()
                        )))
                    }
                };

                let _ = file.write_all(&chunk).await?;
            }
        }
    }

    Ok(HttpResponse::Ok().json(StandardResponse::ok(
        (),
        Some("File saved successfully.".into()),
    )))
}
