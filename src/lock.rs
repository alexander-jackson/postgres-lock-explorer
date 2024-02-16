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
       match value {
            "AccessShareLock" => Ok(Lock::AccessShareLock),
            "RowShareLock" => Ok(Lock::RowShareLock),
            "RowExclusiveLock" => Ok(Lock::RowExclusiveLock),
            "ShareUpdateExclusiveLock" => Ok(Lock::ShareUpdateExclusiveLock),
            "ShareLock" => Ok(Lock::ShareLock),
            "ShareRowExclusiveLock" => Ok(Lock::ShareRowExclusiveLock),
            "ExclusiveLock" => Ok(Lock::ExclusiveLock),
            "AccessExclusiveLock" => Ok(Lock::AccessExclusiveLock),
            _ => {
                let normalised = value.to_lowercase().replace(" ", "");

                let normalised = if !normalised.ends_with("lock") {
                    normalised + "lock"
                } else {
                    normalised
                };

                match normalised.as_str() {
                    "accesssharelock" => Ok(Lock::AccessShareLock),
                    "rowsharelock" => Ok(Lock::RowShareLock),
                    "rowexclusivelock" => Ok(Lock::RowExclusiveLock),
                    "shareupdateexclusivelock" => Ok(Lock::ShareUpdateExclusiveLock),
                    "sharelock" => Ok(Lock::ShareLock),
                    "sharerowexclusivelock" => Ok(Lock::ShareRowExclusiveLock),
                    "exclusivelock" => Ok(Lock::ExclusiveLock),
                    "accessexclusivelock" => Ok(Lock::AccessExclusiveLock),
                    _ => return Err(eyre!("invalid lock type {normalised}")),
                }
            },
        }
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
