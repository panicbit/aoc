use chrono::prelude::*;
use chrono_tz::US::Eastern;
use Result;

pub fn num_unlocked_days(year: u32) -> Result<u8> {
    let december_start = Eastern.ymd(year as i32, 12, 1).and_hms(0, 0, 0);
    let days = Utc::now().signed_duration_since(december_start).num_days() + 1;

    if days <= 0 {
        Ok(0)
    }
    else if days > 25 {
        Ok(25)
    }
    else {
        Ok(days as u8)
    }
}

pub fn unlock_date(year: u32, day: u8) -> Result<Option<DateTime<Local>>> {
    if day < 1 || day > 25 {
        return Ok(None)
    }

    let date = Eastern.ymd(year as i32, 12, day as u32).and_hms(0, 0, 0).with_timezone(&Local);

    Ok(Some(date))
}
