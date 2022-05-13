use actix_web::HttpResponse;

pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[cfg(test)]
mod tests {
    #[actix_web::test]
    async fn health_check_works() {
        let resp = super::health_check().await;
        assert_eq!(resp.status(), actix_web::http::StatusCode::OK);
    }
}
