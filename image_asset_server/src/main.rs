use actix_web::{get, web, App, HttpResponse, HttpServer, Result};
use image::{DynamicImage, ImageFormat};
use serde::Deserialize;
use tokio::fs;

mod config;
use config::ServerConfig;

#[derive(Deserialize)]
struct ImageQuery {
    width: Option<u32>,
    height: Option<u32>,
}

struct ImageServer {
    config: ServerConfig,
}

impl ImageServer {
    pub fn new(config: ServerConfig) -> std::io::Result<Self> {
        config.validate()?;
        Ok(Self { config })
    }

    async fn load_image(&self, filename: &str) -> Option<DynamicImage> {
        let path = self.config.image_dir().join(filename);

        // Prevent directory traversal attacks
        if !path.starts_with(&self.config.image_dir()) {
            return None;
        }

        let bytes = fs::read(&path).await.ok()?;
        image::load_from_memory(&bytes).ok()
    }

    fn validate_dimensions(&self, width: u32, height: u32) -> bool {
        let (max_width, max_height) = self.config.max_dimensions();
        let width_valid = max_width.map_or(true, |max| width <= max);
        let height_valid = max_height.map_or(true, |max| height <= max);
        width_valid && height_valid
    }
}

#[get("/images/{filename}")]
async fn get_image(
    server: web::Data<ImageServer>,
    filename: web::Path<String>,
    query: web::Query<ImageQuery>,
) -> Result<HttpResponse> {
    let img = server
        .load_image(&filename)
        .await
        .ok_or_else(|| actix_web::error::ErrorNotFound("Image not found"))?;

    let img = if let (Some(width), Some(height)) = (query.width, query.height) {
        if !server.validate_dimensions(width, height) {
            return Err(actix_web::error::ErrorBadRequest("Requested dimensions exceed maximum allowed"));
        }
        img.resize_exact(width, height, image::imageops::FilterType::Lanczos3)
    } else {
        img
    };

    let mut bytes: Vec<u8> = Vec::new();
    img.write_to(&mut std::io::Cursor::new(&mut bytes), ImageFormat::Jpeg)
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    
    Ok(HttpResponse::Ok()
        .content_type("image/jpeg")
        .body(bytes))
}

pub async fn run_server(config: ServerConfig, bind_address: &str) -> std::io::Result<()> {
    let server = web::Data::new(ImageServer::new(config)?);

    HttpServer::new(move || {
        App::new()
            .app_data(server.clone())
            .service(get_image)
    })
        .bind(bind_address)?
        .run()
        .await
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = ServerConfig::new("images")
        .with_max_dimensions(2000, 2000);

    run_server(config, "127.0.0.1:8080").await
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, web, App};
    use tempfile::TempDir;

    async fn setup_test_server() -> (TempDir, web::Data<ImageServer>) {
        let temp_dir = TempDir::new().unwrap();
        let config = ServerConfig::new(temp_dir.path())
            .with_max_dimensions(1000, 1000);
        let server = web::Data::new(ImageServer::new(config).unwrap());
        (temp_dir, server)
    }

    #[test]
    async fn test_invalid_config() {
        let config = ServerConfig::new("nonexistent_directory");
        assert!(ImageServer::new(config).is_err());
    }

    #[actix_web::test]
    async fn test_get_nonexistent_image() {
        let (_, server) = setup_test_server().await;

        let app = test::init_service(
            App::new()
                .app_data(server)
                .service(get_image)
        ).await;

        let req = test::TestRequest::get()
            .uri("/images/nonexistent.jpg")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 404);
    }

    #[actix_web::test]
    async fn test_get_existing_image() {
        let (temp_dir, server) = setup_test_server().await;

        // Create a test image
        let test_image = DynamicImage::new_rgb8(100, 100);
        let image_path = temp_dir.path().join("test.jpg");
        test_image.save(&image_path).unwrap();

        let app = test::init_service(
            App::new()
                .app_data(server)
                .service(get_image)
        ).await;

        let req = test::TestRequest::get()
            .uri("/images/test.jpg")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 200);
        assert_eq!(resp.headers().get("content-type").unwrap(), "image/jpeg");
    }

    #[actix_web::test]
    async fn test_resize_image() {
        let (temp_dir, server) = setup_test_server().await;

        // Create a test image
        let test_image = DynamicImage::new_rgb8(100, 100);
        let image_path = temp_dir.path().join("test.jpg");
        test_image.save(&image_path).unwrap();

        let app = test::init_service(
            App::new()
                .app_data(server)
                .service(get_image)
        ).await;

        let req = test::TestRequest::get()
            .uri("/images/test.jpg?width=50&height=50")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 200);

        let body = test::read_body(resp).await;
        let resized_image = image::load_from_memory(&body).unwrap();
        assert_eq!(resized_image.width(), 50);
        assert_eq!(resized_image.height(), 50);
    }

    #[actix_web::test]
    async fn test_resize_image_exceeds_max() {
        let (temp_dir, server) = setup_test_server().await;

        // Create a test image
        let test_image = DynamicImage::new_rgb8(100, 100);
        let image_path = temp_dir.path().join("test.jpg");
        test_image.save(&image_path).unwrap();

        let app = test::init_service(
            App::new()
                .app_data(server)
                .service(get_image)
        ).await;

        let req = test::TestRequest::get()
            .uri("/images/test.jpg?width=2000&height=2000")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 400);
    }

    #[actix_web::test]
    async fn test_directory_traversal_prevention() {
        let (_temp_dir, server) = setup_test_server().await;

        let app = test::init_service(
            App::new()
                .app_data(server)
                .service(get_image)
        ).await;

        let req = test::TestRequest::get()
            .uri("/images/../../../etc/passwd")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 404);
    }
}
