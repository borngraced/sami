use actix_web::middleware::Logger;

pub fn my_logger(format: &str) -> Logger {
    Logger::new(format)
}
