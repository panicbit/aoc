use crate::Result;
use anyhow::Context;
use chrono::prelude::*;
use chrono_tz::US::Eastern;

pub fn num_unlocked_days(year: u32) -> Result<u8> {
    let end_of_november = Eastern
        .with_ymd_and_hms(year as i32, 11, 30, 0, 0, 0)
        .single()
        .context("BUG: Failed to construct date")?;
    let days = Utc::now().signed_duration_since(end_of_november).num_days();

    if days <= 0 {
        Ok(0)
    } else if days > 25 {
        Ok(25)
    } else {
        Ok(days as u8)
    }
}

pub fn unlock_date(year: u32, day: u8) -> Result<Option<DateTime<Local>>> {
    if !(1..=25).contains(&day) {
        return Ok(None);
    }

    let date = Eastern
        .with_ymd_and_hms(year as i32, 12, day as u32, 0, 0, 0)
        .single()
        .context("BUG: Failed to construct date")?
        .with_timezone(&Local);

    Ok(Some(date))
}
