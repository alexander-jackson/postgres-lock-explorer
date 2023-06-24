use std::ops::DerefMut;

use axum::extract::{Json, Path, Query, State};
use backend_connector::{LockAnalysisRequest, LockAnalysisResponse};

use crate::error::ServerResult;
use crate::SharedClient;

pub async fn analyse_locks_on_relation(
    State(state): State<SharedClient>,
    Path(relation): Path<String>,
    Query(req): Query<LockAnalysisRequest>,
) -> ServerResult<Json<Option<LockAnalysisResponse>>> {
    let mut client = state.lock().await;
    let (ref mut left, ref right) = client.deref_mut();

    // Begin a transaction
    let transaction = left.transaction().await?;
    transaction.query(&req.query, &[]).await?;

    // Use the other connection to inspect the locks
    let lock = right
        .query_opt(
            r#"
            SELECT pl.locktype, pl.mode
            FROM pg_locks pl
            JOIN pg_stat_activity psa ON pl.pid = psa.pid
            JOIN pg_class pc ON pc.oid = pl.relation
            WHERE psa.query = $1
            AND pc.relname = $2
        "#,
            &[&req.query, &relation],
        )
        .await?;

    transaction.rollback().await?;

    let response = lock.map(|row| LockAnalysisResponse {
        locktype: row.get(0),
        mode: row.get(1),
        relation,
    });

    Ok(Json(response))
}

pub async fn analyse_all_locks(
    State(state): State<SharedClient>,
    Query(req): Query<LockAnalysisRequest>,
) -> ServerResult<Json<Vec<LockAnalysisResponse>>> {
    let mut client = state.lock().await;
    let (ref mut left, ref right) = client.deref_mut();

    // Begin a transaction
    let transaction = left.transaction().await?;
    transaction.query(&req.query, &[]).await?;

    // Use the other connection to inspect the locks
    let locks = right
        .query(
            r#"
            SELECT pl.locktype, pl.mode, pc.relname
            FROM pg_locks pl
            JOIN pg_stat_activity psa ON pl.pid = psa.pid
            JOIN pg_class pc ON pc.oid = pl.relation
            WHERE psa.query = $1
            ORDER BY pc.relname, pl.mode
        "#,
            &[&req.query],
        )
        .await?;

    transaction.rollback().await?;

    let response = locks
        .iter()
        .map(|row| LockAnalysisResponse {
            locktype: row.get(0),
            mode: row.get(1),
            relation: row.get(2),
        })
        .collect();

    Ok(Json(response))
}
