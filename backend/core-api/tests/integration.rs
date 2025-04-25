use actix_web::{test, App};
use banking_system_backend::core_api::main_app; // function that configures App

#[actix_rt::test]
async fn ft_can_connect() {
    let app = test::init_service(App::new().configure(main_app)).await;
    let req = test::TestRequest::get().uri("/health").to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}
