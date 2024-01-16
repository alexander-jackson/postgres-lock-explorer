use std::fmt;
use std::str::FromStr;

use color_eyre::eyre::{eyre, Report, Result};
use serde::{Deserialize, Serialize};
use tokio_postgres::types::{FromSql, Type};

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub enum Lock {
    AccessShareLock,
    RowShareLock,
    RowExclusiveLock,
    ShareUpdateExclusiveLock,
    ShareLock,
    ShareRowExclusiveLock,
    ExclusiveLock,
    AccessExclusiveLock,
}

impl fmt::Display for Lock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Lock::AccessShareLock => "AccessShareLock",
            Lock::RowShareLock => "RowShareLock",
            Lock::RowExclusiveLock => "RowExclusiveLock",
            Lock::ShareUpdateExclusiveLock => "ShareUpdateExclusiveLock",
            Lock::ShareLock => "ShareLock",
            Lock::ShareRowExclusiveLock => "ShareRowExclusiveLock",
            Lock::ExclusiveLock => "ExclusiveLock",
            Lock::AccessExclusiveLock => "AccessExclusiveLock",
        };

        write!(f, "{value}")
    }
}

impl FromStr for Lock {
    type Err = Report;

    fn from_str(value: &str) -> Result<Self> {
        let lock = match value {
            "AccessShareLock" => Lock::AccessShareLock,
            "RowShareLock" => Lock::RowShareLock,
            "RowExclusiveLock" => Lock::RowExclusiveLock,
            "ShareUpdateExclusiveLock" => Lock::ShareUpdateExclusiveLock,
            "ShareLock" => Lock::ShareLock,
            "ShareRowExclusiveLock" => Lock::ShareRowExclusiveLock,
            "ExclusiveLock" => Lock::ExclusiveLock,
            "AccessExclusiveLock" => Lock::AccessExclusiveLock,
            _ => return Err(eyre!("invalid lock type {value}")),
        };

        Ok(lock)
    }
}

impl FromSql<'_> for Lock {
    fn from_sql(
        ty: &Type,
        raw: &'_ [u8],
    ) -> Result<Self, Box<dyn std::error::Error + Sync + Send>> {
        // &str supports both VARCHAR and TEXT, so this should always work
        let parsed: &str = FromSql::from_sql(ty, raw)?;
        let lock = Lock::from_str(parsed)?;

        Ok(lock)
    }

    fn accepts(ty: &Type) -> bool {
        match *ty {
            Type::VARCHAR | Type::TEXT => true,
            _ => false,
        }
    }
}
