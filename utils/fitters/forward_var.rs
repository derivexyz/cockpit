//! Forward variance curve for quoting US equity options while their market is closed.
//!
//! The curve is constant between listed expiries on a variance clock (a session has mass 1, a night and a weekend
//! their weights), plus a lump of variance per scheduled event. It rebuilds the fitted ATM total variances, to
//! float rounding. Times are UTC epoch seconds, from 1970 to 9999. Total variance is sigma^2 * tau, tau in years
//! of `SEC_PER_YEAR`.

use crate::SEC_PER_YEAR;
use anyhow::{bail, ensure, Context, Result};
use chrono::{DateTime, Datelike, NaiveDate};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::f64::consts::LN_2;
use std::sync::Arc;

const DAY: i64 = 86_400;
const HOUR: i64 = 3_600;
const OPEN_MIN: i64 = 9 * 60 + 30;
const CLOSE_MIN: i64 = 16 * 60;
const EARLY_CLOSE_MIN: i64 = 13 * 60;
// 9999-12-31
const MAX_SEC: i64 = 253_402_300_799;

/// US equity-option sessions, 09:30-16:00 US Eastern on weekdays; dates are epoch days of the Eastern date.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SessionCalendar {
    #[serde(default)]
    pub holidays: BTreeSet<i64>,
    /// session 09:30-13:00
    #[serde(default)]
    pub early_closes: BTreeSet<i64>,
}

/// Clock mass in full sessions. A closure weighs `night` plus `(weekend - night) / 2` per closed date.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClockWeights {
    pub night: f64,
    /// Friday close to Monday open
    pub weekend: f64,
    /// an early-close session
    pub half_day: f64,
    /// cumulative (share of session time, share of its mass) points, laid on the short session on an early close
    #[serde(default)]
    pub session_profile: Option<Vec<(f64, f64)>>,
}

/// Variance released from `start_sec`, fading linearly to zero over `length_sec` (0 is a jump); ignored once over.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Event {
    pub start_sec: i64,
    #[serde(default)]
    pub length_sec: i64,
}

/// After an event the base runs at `1 + excess` times its level; the excess halves every `half_life` of mass.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct PostEvent {
    pub excess: f64,
    pub half_life: f64,
}

/// What to do when an expiry's total variance is below the previous one's.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum NegativeForward {
    #[default]
    Reject,
    /// fit the closest non-decreasing term structure (equal-weight least squares)
    Project,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FwdVarConfig {
    #[serde(default)]
    pub calendar: SessionCalendar,
    pub weights: ClockWeights,
    #[serde(default)]
    pub events: Vec<Event>,
    #[serde(default)]
    pub negative_forward: NegativeForward,
    #[serde(default)]
    pub post_event: Option<PostEvent>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FitFlag {
    /// the knots were replaced by their projection
    Projected,
    /// an event starts after the last expiry and was ignored
    EventDropped,
    /// the donor's base leaves no variance for the events of an interval: they are 0
    NegativeLeftover,
    /// every interval holds an event: the base takes it all
    NoCleanDonor,
}

#[derive(Debug, Clone)]
pub struct ForwardVarModel {
    cfg: Arc<FwdVarConfig>,
}

/// One fit. To move it elsewhere store `ref_sec` and `knots_used`, and fit them again with the same config.
#[derive(Debug, Clone)]
pub struct FittedCurve {
    model: ForwardVarModel,
    ref_sec: i64,
    ends: Vec<i64>,
    cumulative: Vec<f64>,
    members: Vec<Vec<usize>>,
    knots: BTreeMap<i64, f64>,
    base_levels: Vec<f64>,
    starts: Vec<i64>,
    event_sizes: Vec<f64>,
    flags: Vec<FitFlag>,
}

fn epoch_day(year: i32, month: u32, day: u32) -> Result<i64> {
    let date = NaiveDate::from_ymd_opt(year, month, day).context("a date outside the calendar")?;
    Ok((date - NaiveDate::default()).num_days())
}

fn first_sunday(year: i32, month: u32) -> Result<i64> {
    let first = epoch_day(year, month, 1)?;
    Ok(first + (3 - first).rem_euclid(7))
}

/// US Eastern offset under the rules in force since 2007, in seconds east of UTC.
fn eastern_offset(utc: i64) -> Result<i64> {
    let year = DateTime::from_timestamp(utc, 0).context("a time outside the calendar")?.year();
    let key = (utc.div_euclid(DAY), utc.rem_euclid(DAY));
    let spring = (first_sunday(year, 3)? + 7, 7 * HOUR);
    let fall = (first_sunday(year, 11)?, 6 * HOUR);
    Ok(if (spring..fall).contains(&key) {
        -4 * HOUR
    } else {
        -5 * HOUR
    })
}

/// UTC seconds of a daytime minute of an Eastern date.
fn eastern_daytime(day: i64, minute: i64) -> Result<i64> {
    let midnight = day.checked_mul(DAY).context("a date outside the calendar")?;
    Ok(midnight + minute * 60 - eastern_offset(midnight + 12 * HOUR)?)
}

fn is_weekday(day: i64) -> bool {
    (day + 3).rem_euclid(7) < 5
}

/// A session and the closure after it.
struct Cycle {
    day: i64,
    open: i64,
    close: i64,
    next: i64,
    next_open: i64,
    early: bool,
}

impl SessionCalendar {
    fn is_session(&self, day: i64) -> bool {
        is_weekday(day) && !self.holidays.contains(&day)
    }

    fn close_sec(&self, day: i64) -> Result<i64> {
        eastern_daytime(
            day,
            if self.early_closes.contains(&day) {
                EARLY_CLOSE_MIN
            } else {
                CLOSE_MIN
            },
        )
    }

    /// Sessions opening in [first, end).
    fn session_count(&self, first: i64, end: i64) -> i64 {
        if first >= end {
            return 0;
        }
        let days = end - first;
        let weekdays = 5 * (days / 7) + (0..days % 7).filter(|d| is_weekday(first + d)).count() as i64;
        weekdays - self.holidays.range(first..end).filter(|d| is_weekday(**d)).count() as i64
    }

    fn cycle(&self, t: i64) -> Result<Cycle> {
        ensure!((0..=MAX_SEC).contains(&t), "{t} is outside the calendar");
        let mut day = (t + eastern_offset(t)?).div_euclid(DAY);
        if t < eastern_daytime(day, OPEN_MIN)? {
            day -= 1
        }
        while !self.is_session(day) {
            day -= 1
        }
        let mut next = day + 1;
        while !self.is_session(next) {
            next += 1
        }
        Ok(Cycle {
            day,
            open: eastern_daytime(day, OPEN_MIN)?,
            close: self.close_sec(day)?,
            next,
            next_open: eastern_daytime(next, OPEN_MIN)?,
            early: self.early_closes.contains(&day),
        })
    }
}

impl ClockWeights {
    fn per_closed_day(&self) -> f64 {
        (self.weekend - self.night) * 0.5
    }
}

impl Cycle {
    fn session_mass(&self, w: &ClockWeights) -> f64 {
        if self.early {
            w.half_day
        } else {
            1.
        }
    }

    fn closure_mass(&self, w: &ClockWeights) -> f64 {
        w.night + (self.next - self.day - 1) as f64 * w.per_closed_day()
    }

    /// Mass between two times of this cycle.
    fn partial(&self, a: i64, b: i64, w: &ClockWeights) -> f64 {
        let Some(profile) = &w.session_profile else {
            let session = (b.min(self.close) - a.min(self.close)) as f64 / (self.close - self.open) as f64;
            let closed = (b.max(self.close) - a.max(self.close)) as f64 / (self.next_open - self.close) as f64;
            return session * self.session_mass(w) + closed * self.closure_mass(w);
        };
        // constant-density pieces in seconds from the open; a kink may fall between two seconds
        let (lo, hi) = ((a - self.open) as f64, (b - self.open) as f64);
        let duration = (self.close - self.open) as f64;
        let mut total = 0.;
        let mut add = |l: f64, r: f64, mass: f64| {
            if r > l {
                total += mass * ((hi.min(r) - lo.max(l)).max(0.) / (r - l));
            }
        };
        let mut prev = (0., 0.);
        for &(x, y) in profile.iter().chain(&[(1., 1.)]) {
            add(prev.0, x * duration, (y - prev.1) * self.session_mass(w));
            prev = (x * duration, y);
        }
        add(duration, (self.next_open - self.open) as f64, self.closure_mass(w));
        total
    }
}

// equal-weight isotonic least squares; each block holds its mean and count
fn project(values: &mut [f64]) {
    let mut blocks: Vec<(f64, usize)> = Vec::new();
    for mut mean in values.iter().copied() {
        let mut count = 1;
        while let Some(&(left, n)) = blocks.last() {
            if left <= mean {
                break;
            }
            blocks.pop();
            count += n;
            mean += (left - mean) * (n as f64 / count as f64);
        }
        blocks.push((mean, count));
    }
    let mut start = 0;
    for (mean, n) in blocks {
        values[start..start + n].fill(mean);
        start += n;
    }
}

fn flag(flags: &mut Vec<FitFlag>, f: FitFlag) {
    if !flags.contains(&f) {
        flags.push(f)
    }
}

impl ForwardVarModel {
    pub fn new(cfg: FwdVarConfig) -> Result<Self> {
        let w = &cfg.weights;
        ensure!(w.night.is_finite() && w.night > 0., "the night mass must be positive");
        ensure!(
            w.half_day.is_finite() && w.half_day > 0.,
            "the half-day mass must be positive"
        );
        ensure!(
            w.weekend.is_finite() && w.weekend >= w.night,
            "a weekend must weigh at least a night"
        );
        let mut prev = (0., 0.);
        for &(x, y) in w.session_profile.iter().flatten() {
            ensure!(
                x > prev.0 && x < 1. && y >= prev.1 && y <= 1.,
                "the session profile must rise from (0, 0) to (1, 1)"
            );
            prev = (x, y);
        }
        if let Some(p) = cfg.post_event {
            ensure!(
                p.excess.is_finite() && p.excess >= 0.,
                "the post-event excess must not be negative"
            );
            ensure!(
                p.half_life.is_finite() && p.half_life > 0.,
                "the post-event half-life must be positive"
            );
        }
        for e in &cfg.events {
            ensure!(
                e.length_sec >= 0 && e.start_sec.checked_add(e.length_sec).is_some(),
                "invalid event window"
            );
        }
        Ok(Self { cfg: Arc::new(cfg) })
    }

    pub fn config(&self) -> &FwdVarConfig {
        &self.cfg
    }

    /// Clock mass between two times: sessions, and closures at their weights.
    pub fn clock_mass(&self, from_sec: i64, to_sec: i64) -> Result<f64> {
        ensure!(from_sec <= to_sec, "reversed clock interval");
        let (cal, w) = (&self.cfg.calendar, &self.cfg.weights);
        ensure!(to_sec <= MAX_SEC, "{to_sec} is outside the calendar");
        let first = cal.cycle(from_sec)?;
        if to_sec <= first.next_open {
            return Ok(first.partial(from_sec, to_sec, w));
        }
        let last = cal.cycle(to_sec)?;
        let sessions = cal.session_count(first.next, last.day);
        let half = cal.early_closes.range(first.next..last.day).filter(|day| cal.is_session(**day)).count() as i64;
        let closed_days = last.day - first.next - sessions;
        Ok(first.partial(from_sec, first.next_open, w)
            + (sessions - half) as f64
            + half as f64 * w.half_day
            + sessions as f64 * w.night
            + closed_days as f64 * w.per_closed_day()
            + last.partial(last.open, to_sec, w))
    }

    /// Clock mass of [a, b] with the base raised after each event of `starts`.
    fn base_mass(&self, a: i64, b: i64, starts: &[i64]) -> Result<f64> {
        let ordinary = self.clock_mass(a, b)?;
        let Some(p) = self.cfg.post_event.filter(|p| p.excess > 0.) else {
            return Ok(ordinary);
        };
        let mut mass = ordinary;
        for &s in starts.iter().take_while(|&&s| s < b) {
            let age = if s < a { self.clock_mass(s, a)? } else { 0. };
            let span = if s <= a { ordinary } else { self.clock_mass(s, b)? };
            let q = (span / p.half_life) * LN_2;
            // expm1 for short spans, the limiting area for fast decays
            let area = if q == 0. {
                span
            } else if q < 1. {
                span * (-(-q).exp_m1() / q)
            } else {
                p.half_life * (-(-q).exp_m1() / LN_2)
            };
            mass += p.excess * (-age / p.half_life).exp2() * area;
        }
        Ok(mass)
    }

    /// `knots` is expiry (sec) -> ATM total variance from `ref_sec`. An interval that holds an event takes
    /// its base level from the nearest interval without one, the later on a tie; the rest is the event's.
    pub fn fit(&self, ref_sec: i64, knots: &BTreeMap<i64, f64>) -> Result<FittedCurve> {
        let (ends, mut ws): (Vec<_>, Vec<_>) = knots.iter().map(|(&t, &w)| (t, w)).unzip();
        ensure!(
            !ends.is_empty() && ends[0] > ref_sec && ws.iter().all(|w| w.is_finite() && *w > 0.),
            "need positive finite variances at future expiries"
        );
        let mut flags = Vec::new();
        if let Some(i) = ws.windows(2).position(|v| v[1] < v[0]) {
            if self.cfg.negative_forward == NegativeForward::Reject {
                bail!("negative forward variance into expiry {}", ends[i + 1]);
            }
            project(&mut ws);
            flags.push(FitFlag::Projected);
        }
        let mut members = vec![Vec::new(); ends.len()];
        let mut event_sizes = vec![0.; self.cfg.events.len()];
        for (j, e) in self.cfg.events.iter().enumerate() {
            let (t, end) = (e.start_sec, e.start_sec + e.length_sec);
            if end <= ref_sec {
                continue;
            }
            let i = ends.partition_point(|&expiry| expiry < t);
            if i == ends.len() {
                flag(&mut flags, FitFlag::EventDropped);
                continue;
            }
            // it counts in the first expiry at or after its start and must end by it: a split is not identified
            ensure!(
                t > ref_sec && end <= ends[i],
                "the event starting at {t} straddles an expiry or ref_sec"
            );
            members[i].push(j);
        }
        let mut starts: Vec<_> = members.iter().flatten().map(|&j| self.cfg.events[j].start_sec).collect();
        starts.sort_unstable();
        let mut masses = Vec::new();
        let mut levels = Vec::new();
        let (mut prev_t, mut prev_w) = (ref_sec, 0.);
        for (&t, &w) in ends.iter().zip(&ws) {
            let mass = self.base_mass(prev_t, t, &starts)?;
            let level = (w - prev_w) / mass;
            ensure!(
                level.is_finite() && mass > 0. && mass.is_finite(),
                "unrepresentable interval mass or rate"
            );
            masses.push(mass);
            levels.push(level);
            (prev_t, prev_w) = (t, w);
        }
        let clean: Vec<_> = (0..ends.len()).filter(|&i| members[i].is_empty()).collect();
        for (i, events) in members.iter().enumerate().filter(|(_, events)| !events.is_empty()) {
            let Some(donor) = clean.iter().copied().min_by_key(|&d| (i.abs_diff(d), std::cmp::Reverse(d))) else {
                flag(&mut flags, FitFlag::NoCleanDonor);
                continue;
            };
            let prev = if i == 0 { 0. } else { ws[i - 1] };
            let leftover = (ws[i] - prev) - levels[donor] * masses[i];
            if leftover <= 0. {
                flag(&mut flags, FitFlag::NegativeLeftover);
                continue;
            }
            levels[i] = levels[donor];
            for &j in events {
                event_sizes[j] = leftover / events.len() as f64
            }
        }
        // integrated from the fitted levels and sizes, not copied from the knots
        let mut cumulative = vec![0.];
        for (i, &level) in levels.iter().enumerate() {
            let area = members[i].iter().map(|&j| event_sizes[j]).sum::<f64>();
            cumulative.push(cumulative[i] + (level * masses[i] + area));
        }
        Ok(FittedCurve {
            model: self.clone(),
            ref_sec,
            knots: ends.iter().copied().zip(ws).collect(),
            ends,
            cumulative,
            members,
            base_levels: levels,
            starts,
            event_sizes,
            flags,
        })
    }
}

impl FittedCurve {
    pub fn ref_sec(&self) -> i64 {
        self.ref_sec
    }

    /// expiry (sec) -> total variance the curve rebuilds, after any projection.
    pub fn knots_used(&self) -> &BTreeMap<i64, f64> {
        &self.knots
    }

    /// Variance per unit of clock mass, per interval.
    pub fn base_levels(&self) -> &[f64] {
        &self.base_levels
    }

    /// Variance of each event, in the config's order; 0 when it was dropped or left nothing.
    pub fn event_sizes(&self) -> &[f64] {
        &self.event_sizes
    }

    pub fn flags(&self) -> &[FitFlag] {
        &self.flags
    }

    fn cumulative_at(&self, t: i64) -> Result<f64> {
        let i = self.ends.partition_point(|&end| end < t);
        ensure!(
            t >= self.ref_sec && i < self.ends.len(),
            "{t} is outside the fitted range"
        );
        if t == self.ends[i] {
            return Ok(self.cumulative[i + 1]);
        }
        let start = if i == 0 { self.ref_sec } else { self.ends[i - 1] };
        let mut w = self.cumulative[i] + self.base_levels[i] * self.model.base_mass(start, t, &self.starts)?;
        for &j in &self.members[i] {
            let e = self.model.cfg.events[j];
            if t < e.start_sec {
                continue;
            }
            let x = if e.length_sec == 0 {
                1.
            } else {
                ((t - e.start_sec) as f64 / e.length_sec as f64).min(1.)
            };
            w += self.event_sizes[j] * x * (2. - x);
        }
        Ok(w)
    }

    /// Total variance over (t_sec, expiry_sec], a jump at the expiry included.
    pub fn get_total_var(&self, t_sec: i64, expiry_sec: i64) -> Result<f64> {
        ensure!(t_sec <= expiry_sec, "{expiry_sec} is before {t_sec}");
        let w = self.cumulative_at(expiry_sec)? - self.cumulative_at(t_sec)?;
        // rounding across events at one instant can leave it just below zero
        Ok(if w < 0. { 0. } else { w })
    }

    /// Annualised variance rate at `t_sec`, its clock read over the next second; a jump and the last expiry have none.
    pub fn get_forward_var(&self, t_sec: i64) -> Result<f64> {
        let i = self.ends.partition_point(|&end| end <= t_sec);
        ensure!(
            t_sec >= self.ref_sec && i < self.ends.len(),
            "{t_sec} is outside the fitted range"
        );
        // sessions open and close on whole seconds; only a profile point can fall inside the second
        let mut rate = self.base_levels[i] * self.model.clock_mass(t_sec, t_sec + 1)?;
        if let Some(p) = self.model.cfg.post_event.filter(|p| p.excess > 0.) {
            let mut factor = 1.;
            for &s in self.starts.iter().take_while(|&&s| s <= t_sec) {
                factor += p.excess * (-self.model.clock_mass(s, t_sec)? / p.half_life).exp2();
            }
            rate *= factor;
        }
        for &j in &self.members[i] {
            let e = self.model.cfg.events[j];
            if (e.start_sec..e.start_sec + e.length_sec).contains(&t_sec) {
                rate += 2. * self.event_sizes[j] * (1. - (t_sec - e.start_sec) as f64 / e.length_sec as f64)
                    / e.length_sec as f64;
            }
        }
        Ok(rate * SEC_PER_YEAR)
    }
}
