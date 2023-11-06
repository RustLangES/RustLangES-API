use once_cell::sync::Lazy;
use salvo::{prelude::*, cors::Cors, hyper::Method};
use shuttle_salvo::ShuttleSalvo;
use sqlx::PgPool;
use std::collections::HashMap;
use tokio::sync::RwLock;
use tracing::error;

pub static DB_CONNECTION: Lazy<RwLock<Option<PgPool>>> = Lazy::new(|| RwLock::new(None));

#[handler]
async fn count_visit_references(req: &mut Request, res: &mut Response) {
    let Some(reference) = req.query::<String>("reference") else {
        res.status_code(StatusCode::BAD_REQUEST);
        return;
    };
    let reference = reference.to_lowercase();
    let db = DB_CONNECTION.write().await.clone();

    if let Ok(result) = sqlx::query!("SELECT id FROM domains WHERE domain = $1", reference)
        .fetch_one(&db.clone().unwrap())
        .await
    {
        if let Err(error) = sqlx::query!("INSERT INTO visits (domain_id) VALUES ($1)", result.id)
            .execute(&db.unwrap())
            .await
        {
            error!("Error inserting visit: {:#?}", error);
            res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
            return;
        }
        res.status_code(StatusCode::OK);
        return;
    }

    match sqlx::query!(
        "INSERT INTO domains (domain) VALUES ($1) RETURNING id",
        reference
    )
    .fetch_one(&db.clone().unwrap())
    .await
    {
        Err(error) => {
            error!("Error inserting domain: {:#?}", error);
            res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
            return;
        }
        Ok(result) => {
            if let Err(error) =
                sqlx::query!("INSERT INTO visits (domain_id) VALUES ($1)", result.id)
                    .execute(&db.unwrap())
                    .await
            {
                error!("Error inserting visit: {:#?}", error);
                res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
                return;
            }
        }
    };

    res.status_code(StatusCode::OK);
}

#[handler]
async fn list_visit_references(res: &mut Response) {
    let db = DB_CONNECTION.write().await.clone();

    match sqlx::query!(
        "SELECT domain, COUNT(visits.id) as visits 
        FROM domains 
        INNER JOIN visits ON domains.id = visits.domain_id 
        GROUP BY domain"
    )
    .fetch_all(&db.unwrap())
    .await
    {
        Err(error) => {
            error!("Error listing domains: {:#?}", error);
            res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
            return;
        }
        Ok(result) => {
            let mut references = HashMap::with_capacity(result.len());
            for row in result {
                references.insert(row.domain, row.visits);
            }
            res.render(Json(references));
        }
    };
}

#[shuttle_runtime::main]
async fn salvo(
    #[shuttle_shared_db::Postgres(
        local_uri = "postgres://postgres:{secrets.PASSWORD}@localhost:16695/RustLangEs"
    )]
    pool: PgPool,
) -> shuttle_salvo::ShuttleSalvo {
    if let Err(e) = sqlx::migrate!("./migrations").run(&pool).await {
        println!("Error running migrations: {:#?}", e);
        return ShuttleSalvo::Err(shuttle_runtime::Error::Custom(e.into()));
    }
    
    *DB_CONNECTION.write().await = Some(pool);
    let cors_handler = Cors::new()
        .allow_origin("https://rustlanges.github.io")
        .allow_methods(vec![Method::GET, Method::POST])
        .into_handler();
    
    let mut router = Router::with_hoop(cors_handler);
    router = router.push(
        Router::with_path("track")
            .push(Router::with_path("count").post(count_visit_references))
            .get(list_visit_references),
    );

    Ok(router.into())
}
