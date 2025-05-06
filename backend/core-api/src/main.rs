// /home/inno/elights_jobes-research/backend/core-api/src/main.rs

// Use static CONFIG loaded via Lazy
use core_api::config::app_config::CONFIG;
use core_api::config::tls_config::load_server_tls_config;
use core_api::db::{init_db_pool};
use core_api::middlewares::{auth_guard::AuthGuard, logger::RequestLogger}; // Added AuthGuard
use core_api::routes::configure_routes;
use core_api::services::ft_client::FtApiClient; // Import FT Client
use core_api::utils::http_clients::{init_http_clients, HttpClients}; // Import HTTP Clients

use actix_cors::Cors; // Import CORS
use actix_web::{middleware::Logger as ActixLogger, web, App, HttpServer}; // Use ActixLogger alias
use std::sync::Arc;

// Import other necessary crates/modules
use cryptography_exchange::btcpay::BTCPayClient;
use cryptography_exchange::monero_wallet::MoneroWalletRpcClient;
// Import bank clients (example)
use bank_integrations::usa::{ChaseClient, WellsFargoClient};
use bank_integrations::europe::{DeutscheBankClient, BnpParibasClient};


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // .env is loaded implicitly by Lazy<AppConfig> calling AppConfig::load()
    // Initialize logger (uses RUST_LOG from .env)
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("--- Starting Elights Core API Service ---");

    // --- Configuration Loading (using static CONFIG) ---
    // Access config values via CONFIG static ref, e.g., CONFIG.database_url
    log::info!("Configuration loaded successfully.");
    log::debug!("AppConfig: {:?}", *CONFIG); // Log config only in debug mode

    // --- Initialize Database Pool ---
    let db_pool = init_db_pool(&CONFIG.database_url)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?; // Convert ApiError to io::Error
    log::info!("Database connection pool initialized.");

    // --- Initialize Shared HTTP Clients (Standard & Tor) ---
    let http_clients = init_http_clients(&CONFIG)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    log::info!("Shared HTTP clients initialized.");

    // --- Initialize External Service Clients ---
    // These clients often need config values (API Keys, URLs) and potentially an HTTP client
    // TODO: Handle client initialization errors gracefully
    let btcpay_client = BTCPayClient::new(Some(CONFIG.btcpay_default_store_id.clone()))
        .expect("Failed to initialize BTCPay Client");
    log::info!("BTCPay Client initialized.");

    // Monero client might fail if RPC isn't running, handle optional init?
    #[cfg(feature = "monero_support")]
    let monero_client = MoneroWalletRpcClient::new()
         .expect("Failed to initialize Monero Wallet RPC Client");
    #[cfg(feature = "monero_support")]
    log::info!("Monero Wallet RPC Client initialized.");

    // Example Bank Clients
    // TODO: Handle errors for each client individually
    let chase_client = ChaseClient::new().expect("Failed to init Chase Client");
    let wf_client = WellsFargoClient::new().expect("Failed to init Wells Fargo Client");
    let db_client = DeutscheBankClient::new().expect("Failed to init Deutsche Bank Client");
    let bnp_client = BnpParibasClient::new().expect("Failed to init BNP Paribas Client");
    log::info!("Bank integration clients initialized.");

    // Financial Times API Client
    let ft_client = FtApiClient::new(
        CONFIG.ft_api_key.clone(),
        http_clients.standard_client.clone() // Use standard client for FT
    );
    log::info!("FT API Client initialized.");


    // --- Load TLS Configuration (if enabled) ---
    let rustls_config = if CONFIG.tls_enabled {
        log::info!("TLS is ENABLED. Loading certificates...");
        match load_server_tls_config(&CONFIG.tls_cert_path, &CONFIG.tls_key_path) {
            Ok(cfg) => {
                log::info!("TLS configuration loaded successfully.");
                Some(cfg)
            },
            Err(e) => {
                 log::error!("Failed to load TLS configuration: {}. Server will start without TLS.", e);
                 None // Fallback to no TLS if loading fails
            }
        }
    } else {
        log::info!("TLS is DISABLED.");
        None
    };

    // --- Create Shared State (Arc for thread safety) ---
    let app_config = Arc::new(CONFIG.clone()); // Share config via Arc
    let shared_db_pool = web::Data::new(db_pool.clone());
    let shared_http_clients = web::Data::new(http_clients.clone());
    let shared_btcpay = web::Data::new(btcpay_client);
    #[cfg(feature = "monero_support")]
    let shared_monero = web::Data::new(monero_client);
    let shared_ft_client = web::Data::new(ft_client);
    // Share bank clients
    let shared_chase = web::Data::new(chase_client);
    let shared_wf = web::Data::new(wf_client);
    let shared_db = web::Data::new(db_client);
    let shared_bnp = web::Data::new(bnp_client);


    // --- Start Actix HTTP Server ---
    let server_address = &app_config.api_bind_address;
    log::info!("🚀 Starting Core API server at http(s)://{}", server_address);

    let mut http_server = HttpServer::new(move || {
        // Configure CORS (adjust origins, methods, headers as needed for your frontend)
        let cors = Cors::default()
            .allow_any_origin() // Example: Allow any origin (Restrict in production!)
            // .allowed_origin("https://yourfrontend.com") // Production example
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            // --- Share Application State ---
            .app_data(shared_db_pool.clone())
            .app_data(web::Data::from(app_config.clone())) // Share Arc<AppConfig>
            .app_data(shared_http_clients.clone())
            // Share external service clients
            .app_data(shared_btcpay.clone())
            #[cfg(feature = "monero_support")]
            .app_data(shared_monero.clone())
            .app_data(shared_ft_client.clone())
            // Share bank clients
            .app_data(shared_chase.clone())
            .app_data(shared_wf.clone())
            .app_data(shared_db.clone())
            .app_data(shared_bnp.clone())
            // TODO: Add other shared clients (US Bank, Citi, PNC, ING, Santander, Barclays, HSBC)


            // --- Register Middleware ---
            .wrap(cors) // Enable CORS
            .wrap(ActixLogger::default()) // Default concise access logger
            .wrap(RequestLogger::default()) // Your custom detailed logger
            // Note: AuthGuard is applied per-route/scope in routes/*.rs where needed
            // Note: IPWhitelist middleware can be added here globally if desired
            // .wrap(IpWhitelist::new(app_config.allowed_ips.iter().map(|ip| ip.to_string()).collect()))


            // --- Register API Routes ---
            .configure(configure_routes)
    });

    // Bind TLS if configured
    http_server = if let Some(tls_cfg) = rustls_config {
        let tls_bind_address = app_config.tls_bind_address.as_deref()
            .unwrap_or(server_address); // Use same address if TLS bind addr not specified
        log::info!("Binding TLS to {}", tls_bind_address);
        http_server.bind_rustls_0_21(tls_bind_address, tls_cfg)? // Use bind_rustls_0_21 for rustls 0.21
    } else {
        http_server
    };

    // Bind non-TLS address
    http_server = http_server.bind(server_address)?;

    log::info!("Server running. Press Ctrl+C to stop.");
    http_server.run().await
}