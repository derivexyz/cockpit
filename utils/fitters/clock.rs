use anyhow::{ensure, Context, Result};
use chrono::{DateTime, Datelike, NaiveDate};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

/// How fast variance time runs against wall time.
pub trait VarClock {
    /// Constant-rate pieces of [from_sec, to_sec): (start_sec, clock mass per second), in order from from_sec.
    fn weights(&self, from_sec: i64, to_sec: i64) -> Result<Vec<(i64, f64)>>;
}

/// US equity-option sessions, 09:30 to 16:00 US Eastern on weekdays.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UsEquityClock {
    /// Mass of an ordinary overnight closure.
    pub night: f64,
    /// Mass from a regular Friday close to the following Monday open.
    pub weekend: f64,
    /// Mass of a session ending at 13:00 Eastern.
    pub half_day: f64,
    /// Eastern epoch days on which the market is closed.
    #[serde(default)]
    pub holidays: BTreeSet<i64>,
    /// Eastern epoch days on which the session ends early.
    #[serde(default)]
    pub early_closes: BTreeSet<i64>,
}

const DAY: i64 = 86_400;
const LAST_SEC: i64 = 253_402_300_799;

fn epoch_day(date: NaiveDate) -> i64 {
    (date - NaiveDate::default()).num_days()
}

// sessions open after the sunday clock change and on the same UTC and Eastern date
fn open_sec(date: NaiveDate) -> i64 {
    let last_sunday = date.day() as i32 - date.weekday().num_days_from_sunday() as i32;
    let daylight = match date.month() {
        3 => last_sunday >= 8,
        4..=10 => true,
        11 => last_sunday < 1,
        _ => false,
    };
    let hour = if daylight { 13 } else { 14 };
    epoch_day(date) * DAY + hour * 3600 + 30 * 60
}

impl UsEquityClock {
    fn session_date(&self, mut date: NaiveDate, step: fn(&NaiveDate) -> Option<NaiveDate>) -> Result<NaiveDate> {
        while date.weekday().num_days_from_monday() >= 5 || self.holidays.contains(&epoch_day(date)) {
            date = step(&date).context("a closure extends outside the calendar")?;
        }
        Ok(date)
    }
}

impl VarClock for UsEquityClock {
    fn weights(&self, from_sec: i64, to_sec: i64) -> Result<Vec<(i64, f64)>> {
        ensure!(
            self.night.is_finite() && self.night > 0.,
            "a night must have positive finite mass"
        );
        ensure!(
            self.half_day.is_finite() && self.half_day > 0.,
            "a half-day must have positive finite mass"
        );
        ensure!(
            self.weekend.is_finite() && self.weekend >= self.night,
            "a weekend must weigh at least a night"
        );
        ensure!(
            0 <= from_sec && from_sec <= to_sec && to_sec <= LAST_SEC,
            "invalid clock interval"
        );
        let mut pieces = Vec::new();
        if from_sec == to_sec {
            return Ok(pieces);
        }

        let mut date = DateTime::from_timestamp(from_sec, 0).context("a time outside the calendar")?.date_naive();
        if from_sec < open_sec(date) {
            date = date.pred_opt().context("a date outside the calendar")?;
        }
        date = self.session_date(date, NaiveDate::pred_opt)?;
        let mut start = from_sec;
        while start < to_sec {
            let open = open_sec(date);
            let early = self.early_closes.contains(&epoch_day(date));
            let (duration, mass) = if early { (12_600, self.half_day) } else { (23_400, 1.) };
            let close = open + duration;
            if start < close {
                let rate = mass / duration as f64;
                pieces.push((start, rate));
                if to_sec <= close {
                    break;
                }
                start = close;
            }
            let next = self.session_date(
                date.succ_opt().context("a date outside the calendar")?,
                NaiveDate::succ_opt,
            )?;
            let next_open = open_sec(next);
            let closed_days = (next - date).num_days() - 1;
            let seconds = (next_open - close) as f64;
            let rate = self.night / seconds + (self.weekend - self.night) * (0.5 * closed_days as f64 / seconds);
            pieces.push((start, rate));
            start = next_open;
            date = next;
        }
        Ok(pieces)
    }
}
