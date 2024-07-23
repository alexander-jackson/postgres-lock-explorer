use std::ops::DerefMut;

use axum::extract::{Json, State};
use tokio_postgres::types::ToSql;
use tokio_postgres::Client;

use crate::server::error::ServerResult;
use crate::server::SharedClient;
use crate::types::{LockAnalysisRequest, LockAnalysisResponse};

pub async fn analyse_locks(
    State(state): State<SharedClient>,
    Json(req): Json<LockAnalysisRequest>,
) -> ServerResult<Json<Vec<LockAnalysisResponse>>> {
    let mut client = state.lock().await;
    let (ref mut left, ref right) = client.deref_mut();

    let LockAnalysisRequest {
        query,
        schema,
        relation,
    } = &req;

    let lock_query = r#"
        SELECT pl.locktype, pl.mode, pn.nspname, pc.relname
        FROM pg_locks pl
        JOIN pg_stat_activity psa ON pl.pid = psa.pid
        JOIN pg_class pc ON pc.oid = pl.relation
        JOIN pg_namespace pn ON pn.oid = pc.relnamespace
        WHERE psa.query = $1
        AND ($2::TEXT IS NULL OR pn.nspname = $2)
        AND ($3::TEXT IS NULL OR pc.relname = $3)
        ORDER BY pc.relname, pl.mode
    "#;

    tracing::info!(?query, "Analysing all locks");

    let locks = inspect_locks(left, right, query, lock_query, &[query, schema, relation]).await?;

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
    transaction.simple_query(query).await?;

    // Use the other connection to inspect the locks
    let locks = right.query(lock_query, lock_query_params).await?;

    transaction.rollback().await?;

    let response = locks
        .iter()
        .map(|row| LockAnalysisResponse {
            locktype: row.get(0),
            mode: row.get(1),
            schema: row.get(2),
            relation: row.get(3),
        })
        .collect();

    Ok(response)
}
