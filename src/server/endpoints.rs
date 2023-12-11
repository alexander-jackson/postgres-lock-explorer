use std::ops::DerefMut;

use axum::extract::{Json, Path, State};
use tokio_postgres::types::ToSql;
use tokio_postgres::Client;

use crate::server::error::ServerResult;
use crate::server::SharedClient;
use crate::types::{LockAnalysisRequest, LockAnalysisResponse};

pub async fn analyse_locks_on_relation(
    State(state): State<SharedClient>,
    Path(relation): Path<String>,
    Json(req): Json<LockAnalysisRequest>,
) -> ServerResult<Json<Vec<LockAnalysisResponse>>> {
    let mut client = state.lock().await;
    let (ref mut left, ref right) = client.deref_mut();

    let lock_query = r#"
        SELECT pl.locktype, pl.mode, pc.relname
        FROM pg_locks pl
        JOIN pg_stat_activity psa ON pl.pid = psa.pid
        JOIN pg_class pc ON pc.oid = pl.relation
        WHERE psa.query = $1
        AND pc.relname = $2
    "#;

    let locks = inspect_locks(
        left,
        right,
        &req.query,
        lock_query,
        &[&req.query, &relation],
    )
    .await?;

    Ok(Json(locks))
}

pub async fn analyse_all_locks(
    State(state): State<SharedClient>,
    Json(req): Json<LockAnalysisRequest>,
) -> ServerResult<Json<Vec<LockAnalysisResponse>>> {
    let mut client = state.lock().await;
    let (ref mut left, ref right) = client.deref_mut();

    let lock_query = r#"
        SELECT pl.locktype, pl.mode, pc.relname
        FROM pg_locks pl
        JOIN pg_stat_activity psa ON pl.pid = psa.pid
        JOIN pg_class pc ON pc.oid = pl.relation
        WHERE psa.query = $1
        ORDER BY pc.relname, pl.mode
    "#;

    let locks = inspect_locks(left, right, &req.query, lock_query, &[&req.query]).await?;

    Ok(Json(locks))
}

async fn inspect_locks(
    left: &mut Client,
    right: &Client,
    query: &str,
    lock_query: &str,
    lock_query_params: &[&(dyn ToSql + Sync)],
) -> ServerResult<Vec<LockAnalysisResponse>> {
    // Begin a transaction
    let transaction = left.transaction().await?;
    transaction.query(query, &[]).await?;

    // Use the other connection to inspect the locks
    let locks = right.query(lock_query, lock_query_params).await?;

    transaction.rollback().await?;

    let response = locks
        .iter()
        .map(|row| LockAnalysisResponse {
            locktype: row.get(0),
            mode: row.get(1),
            relation: row.get(2),
        })
        .collect();

    Ok(response)
}
