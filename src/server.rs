use actix_cors::Cors;
use actix_web::{web, App, HttpServer, HttpResponse, Responder, middleware};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Instant;

// Bring in the pristine Phase 5 engine components
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::semantic::Analyzer;
use crate::codegen::{Evaluator, RuntimeValue};

// ---------------------------------------------------------
// STRUCTURED DATA CONTRACTS
// ---------------------------------------------------------
#[derive(Deserialize)]
pub struct YaraRequest {
    pub code: String,
}

#[derive(Serialize)]
pub struct YaraResponse {
    pub success: bool,
    pub execution_time_ms: u128,
    pub memory_state: Option<HashMap<String, String>>,
    pub error_details: Option<String>,
}

fn format_runtime_value(val: &RuntimeValue) -> String {
    match val {
        RuntimeValue::Money { amount, currency } => format!("{} {}", amount, currency),
        RuntimeValue::Boolean(b) => b.to_string(),
    }
}

// ---------------------------------------------------------
// THE CORE API CONTROLLER
// ---------------------------------------------------------
async fn execute_yarat(req: web::Json<YaraRequest>) -> impl Responder {
    let start_time = Instant::now();

    // 1. Lex and Parse
    let mut lexer = Lexer::new(&req.code);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();

    // 2. Semantic Audit
    let mut analyzer = Analyzer::new();
    if let Err(audit_error) = analyzer.analyze_program(&program) {
        log::error!("Security Audit Failed: {}", audit_error);
        return HttpResponse::BadRequest().json(YaraResponse {
            success: false,
            execution_time_ms: start_time.elapsed().as_millis(),
            memory_state: None,
            error_details: Some(audit_error),
        });
    }

    // 3. Execution
    let mut evaluator = Evaluator::new();
    evaluator.evaluate_program(&program);

    // 4. State Extraction
    let mut final_memory = HashMap::new();
    for (key, val) in &evaluator.memory {
        final_memory.insert(key.clone(), format_runtime_value(val));
    }

    log::info!("Successfully executed YaraT payload in {} ms", start_time.elapsed().as_millis());

    HttpResponse::Ok().json(YaraResponse {
        success: true,
        execution_time_ms: start_time.elapsed().as_millis(),
        memory_state: Some(final_memory),
        error_details: None,
    })
}

// ---------------------------------------------------------
// PRODUCTION SERVER BOOTSTRAP
// ---------------------------------------------------------
pub async fn run_server() -> std::io::Result<()> {
    // Initialize standard terminal telemetry
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    log::info!("🚀 Booting YaraT Enterprise Web Engine...");
    log::info!("📡 Listening securely on http://127.0.0.1:8080");

    // Configure memory protection (Limit payloads to 4KB to prevent OOM attacks)
    let json_config = web::JsonConfig::default()
        .limit(4096)
        .error_handler(|err, _req| {
            log::warn!("Payload rejected: {}", err);
            actix_web::error::InternalError::from_response(
                err,
                HttpResponse::BadRequest().json(YaraResponse {
                    success: false,
                    execution_time_ms: 0,
                    memory_state: None,
                    error_details: Some("Payload limit exceeded or invalid JSON structure.".to_string()),
                }),
            ).into()
        });

    HttpServer::new(move || {
        // Configure strict Network Security (CORS)
        let cors = Cors::default()
            .allow_any_origin() // In production, restrict this to specific frontend URLs
            .allowed_methods(vec!["POST"])
            .allowed_headers(vec![actix_web::http::header::CONTENT_TYPE])
            .max_age(3600);

        App::new()
            .wrap(middleware::Compress::default()) // Bandwidth optimization
            .wrap(middleware::Logger::default())   // Request telemetry
            .wrap(cors)                            // Network security
            .app_data(json_config.clone())         // Memory protection
            .route("/api/v1/execute", web::post().to(execute_yarat))
    })
    .bind("127.0.0.1:8080")?
    .workers(4) // Multithreading: Allocate 4 dedicated CPU cores to handle concurrent traffic
    .client_request_timeout(std::time::Duration::from_secs(5)) // Prevent slow-loris network attacks
    .run()
    .await
}
