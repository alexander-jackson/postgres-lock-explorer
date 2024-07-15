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
            "AccessShareLock" | "AccessShare" | "ACCESS SHARE" => Self::AccessShareLock,
            "RowShareLock" | "RowShare" | "ROW SHARE" => Self::RowShareLock,
            "RowExclusiveLock" | "RowExclusive" | "ROW EXCLUSIVE" => Self::RowExclusiveLock,
            "ShareUpdateExclusiveLock" | "ShareUpdateExclusive" | "SHARE UPDATE EXCLUSIVE" => {
                Self::ShareUpdateExclusiveLock
            }
            "ShareLock" | "Share" | "SHARE" => Self::ShareLock,

            "ShareRowExclusiveLock" | "ShareRowExclusive" | "SHARE ROW EXCLUSIVE" => {
                Self::ShareRowExclusiveLock
            }
            "ExclusiveLock" | "Exclusive" | "EXCLUSIVE" => Self::ExclusiveLock,
            "AccessExclusiveLock" | "AccessExclusive" | "ACCESS EXCLUSIVE" => {
                Self::AccessExclusiveLock
            }

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
        let lock = Self::from_str(parsed)?;

        Ok(lock)
    }

    fn accepts(ty: &Type) -> bool {
        match *ty {
            Type::VARCHAR | Type::TEXT => true,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use color_eyre::eyre::Result;
    use rstest::rstest;

    use crate::lock::Lock;

    #[rstest]
    #[case::postgres_syntax("AccessShareLock", Lock::AccessShareLock)]
    #[case::without_lock_suffix("AccessShare", Lock::AccessShareLock)]
    #[case::explicit_acquiry("ACCESS SHARE", Lock::AccessShareLock)]
    fn can_parse_lock_types(#[case] input: &str, #[case] expected: Lock) -> Result<()> {
        let actual = Lock::from_str(input)?;

        assert_eq!(actual, expected);

        Ok(())
    }
}
