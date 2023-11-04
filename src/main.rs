use once_cell::sync::Lazy;
use salvo::prelude::*;
use shuttle_persist::PersistInstance;
use tokio::sync::RwLock;

pub static PERSIST_CONNECTION: Lazy<RwLock<Option<PersistInstance>>> = Lazy::new(|| RwLock::new(None));

#[handler]
async fn count_visit_references(req: &mut Request, res: &mut Response) {
    let Some(reference) = req.query::<String>("reference") else {
        res.status_code(StatusCode::BAD_REQUEST);
        return;
    };
    let reference = reference.to_lowercase();
    let persist = PERSIST_CONNECTION.write().await.clone().unwrap();
    
    let visits = match persist.load::<u32>(&reference.clone()) {
        Ok(value) => value,
        Err(_) => 0
    };

    match persist.save(&reference, visits + 1) {
        Ok(_) => res.status_code(StatusCode::OK),
        Err(_) => res.status_code(StatusCode::INTERNAL_SERVER_ERROR)
    };
}

#[handler]
async fn list_visit_references(res: &mut Response) {
    let persist = PERSIST_CONNECTION.write().await.clone().unwrap();
    let references = match persist.list() {
        Ok(value) => value,
        Err(_) => Vec::new()
    };

    let references = references.iter().map(|reference| {
        let visits = match persist.load::<u32>(&reference) {
            Ok(value) => value,
            Err(_) => 0
        };
        (reference, visits)
    }).collect::<Vec<(&String, u32)>>();

    res.render(Json(references));
}

#[shuttle_runtime::main]
async fn salvo(
    #[shuttle_persist::Persist] persist: PersistInstance,
) -> shuttle_salvo::ShuttleSalvo {
    *PERSIST_CONNECTION.write().await = Some(persist);
    let mut router = Router::new();
    router = router.push(
        Router::with_path("track")
        .push(
            Router::with_path("count")
            .post(count_visit_references)
        )
        .get(list_visit_references)
    );

    Ok(router.into())
}
