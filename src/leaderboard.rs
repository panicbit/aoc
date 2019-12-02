use reqwest::Client;
use reqwest::header::COOKIE;
use std::collections::BTreeMap;
use failure::ResultExt;
use chrono::prelude::*;
use chrono::Duration;
use {util, Result};

#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct Leaderboard {
    owner_id: String,
    event: String,
    members: BTreeMap<String, Member>,
}

impl Leaderboard {
    pub fn owner_id(&self) -> &str {
        &self.owner_id
    }

    pub fn event(&self) -> &str {
        &self.event
    }

    pub fn fetch(leaderboard_url: &str, session_token: &str) -> Result<Leaderboard> {
        let client = Client::new();
        let cookie = format!("session={}", session_token);

        let mut resp = client
            .get(leaderboard_url)
            .header(COOKIE, cookie)
            .send()?
            .error_for_status()?;

        let leaderboard = resp.json::<Leaderboard>()?;

        Ok(leaderboard)
    }

    pub fn members<'a>(&'a self) -> Box<Iterator<Item=&'a Member> + 'a> {
        Box::new(self.members.values())
    }

    fn year(&self) -> Result<u32> {
        let year = self.event.parse::<u32>()
            .context("Event name is not a valid year")?;
        Ok(year)
    }

    pub fn num_unlocked_days(&self) -> Result<u8> {
        let year = self.year()?;
        util::num_unlocked_days(year)
    }

    pub fn next_unlock_date(&self) -> Result<Option<DateTime<Local>>> {
        let year = self.year()?;
        let num_unlocked_days = self.num_unlocked_days()?;
        let next_locked_day = num_unlocked_days + 1;

        util::unlock_date(year, next_locked_day)
    }

    pub fn duration_until_next_unlock(&self) -> Result<Option<Duration>> {
        Ok(self.next_unlock_date()?.map(|unlock_date|
            unlock_date.signed_duration_since(Utc::now())
        ))
    }
}

#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct Member {
    id: String,
    name: Option<String>,
    global_score: u32,
    local_score: u32,
    stars: u32,
    completion_day_level: BTreeMap<String, Level>,
    #[serde(with = "ts")]
    last_star_ts: DateTime<Local>,
}

impl Member {
    pub fn name(&self) -> &str {
        match &self.name {
            Some(name) => name,
            None => &self.id,
        }
    }

    pub fn completed_days(&self) -> &BTreeMap<String, Level> {
        &self.completion_day_level
    }

    pub fn local_score(&self) -> u32 {
        self.local_score
    }
}

#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct Level {
    #[serde(rename="1")]
    one: StarInfo,
    #[serde(rename="2")]
    two: Option<StarInfo>,
}

impl Level {
    pub fn one(&self) -> &StarInfo {
        &self.one
    }

    pub fn two(&self) -> Option<&StarInfo> {
        self.two.as_ref()
    }
}

#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct StarInfo {
    #[serde(with = "ts")]
    get_star_ts: DateTime<Local>,
}

impl StarInfo {
    pub fn date(&self) -> DateTime<Local> {
        self.get_star_ts
    }
}

mod ts {
    use serde::{ser, de, Deserialize, Serialize};
    use serde::de::Error;
    use chrono::{TimeZone, DateTime, Local};

    #[derive(Deserialize)]
    #[serde(untagged)]
    enum Ts {
        Int(i64),
        String(String),
    }

    pub fn deserialize<'de, D>(de: D) -> Result<DateTime<Local>, D::Error>
        where D: de::Deserializer<'de>
    {
        let ts = Ts::deserialize(de)?;
        let ts = match ts {
            Ts::Int(ts) => ts,
            Ts::String(ts) => ts.parse::<i64>().map_err(<_>::custom)?,
        };
        let date = Local.timestamp_opt(ts, 0)
            .single()
            .ok_or_else(|| <_>::custom("invalid timestamp"))?;

        Ok(date)
    }

    pub fn serialize<S>(date: &DateTime<Local>, ser: S) -> Result<S::Ok, S::Error>
        where S: ser::Serializer
    {
        let ts = date.timestamp();
        let ts = ts.to_string();

        if ts == "0" {
            0u8.serialize(ser)
        } else {
            ts.serialize(ser)
        }
    }
}
