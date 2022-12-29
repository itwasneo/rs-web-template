mod errors;

use actix_web::{get, middleware, web, web::Data, App, HttpResponse, HttpServer, Responder};
use errors::ApiError;
use tracing::error;

struct AppState {
    {% if redis != "none" %}
    _redis_pool: r2d2::Pool<r2d2_redis::RedisConnectionManager>,
    {% endif %}
    _postgres_pool: sqlx::Pool<sqlx::Postgres>,
}

#[get("/hello")]
async fn hello(_data: web::Data<AppState>) -> impl Responder {
    HttpResponse::Ok().body("hello world")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Setting up tracing subscriber for application logs
    // This subscriber will be used to process traces emitted
    // after this point
    let subscriber = tracing_subscriber::fmt()
        .compact()
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_target(false)
        .finish();

    // Couldn't setting up "tracer" makes the program panic.
    tracing::subscriber::set_global_default(subscriber).unwrap_or_else(|e| {
        error!("{}", ApiError::SetGlobalTracerError(e.into()));
        panic!("FATAL ERROR");
    });

    {% if use_dotenv %}
    // Getting Environment variables
    dotenv::dotenv().ok();
    {% endif %}

    {% if redis != "none" %}
    // Setting Datasources
    // If any of the Datasources couldn't get initialized appropriately the
    // program panic.
    let manager = r2d2_redis::RedisConnectionManager::new(
        {% if use_dotenv %}
        format!("{}:{}",
        &std::env::var("REDIS_HOST").unwrap(),
        &std::env::var("REDIS_PORT").unwrap())
        {% else %}
        "redis://127.0.0.1:6379",
        {% endif %}
    )
    .unwrap_or_else(|e| {
        error!("{}", ApiError::RedisConnectionError(e.into()));
        panic!("FATAL ERROR");
    });

    let redis_pool = r2d2_redis::r2d2::Pool::builder()
        .connection_timeout(std::time::Duration::from_secs(1))
        .idle_timeout(Some(std::time::Duration::from_secs(2)))
        .max_size(20)
        .build(manager)
        .unwrap_or_else(|e| {
            error!("{}", ApiError::RedisConnectionPoolError(e.into()));
            panic!("FATAL ERROR")
        });
    {% endif %}

    let postgres_pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(20)
        .acquire_timeout(std::time::Duration::from_secs(1))
        .idle_timeout(std::time::Duration::from_secs(2))
        .connect(
            {% if use_dotenv %}
            &std::env::var("DATABASE_URL").unwrap()
            {% else %}
            "postgres://user:password@localhost/db_name",
            {% endif %}
        )
        .await
        .unwrap_or_else(|e| {
            error!("{}", ApiError::PostgresConnectionPoolError(e.into()));
            panic!("FATAL ERROR")
        });

    HttpServer::new(move || {
        {% if use_cors %}
        let cors = actix_cors::Cors::default().allowed_origin(
            {% if use_dotenv %}
            &std::env::var("ALLOWED_ORIGIN").unwrap()
            {% else %}
            "http://localhost:8081",
            {% endif %}
        );
        {% endif %}
        App::new()
            .app_data(Data::new(AppState {
                {% if redis != "none" %}
                _redis_pool: redis_pool.clone(),
                {% endif %}
                _postgres_pool: postgres_pool.clone(),
            }))
            .wrap(middleware::Compress::default())
            {% if use_cors %}
            .wrap(cors)
            {% endif %}
            .service(web::scope("/api").service(hello))
    })
    .bind(
        {% if use_dotenv %}
        (
            std::env::var("SERVER_HOST").unwrap(),
            std::env::var("SERVER_PORT")
                .unwrap()
                .parse::<u16>()
                .unwrap(),
        )
        {% else %}
        ("127.0.0.1", 8080),
        {% endif %}
    )?
    .run()
    .await
}
