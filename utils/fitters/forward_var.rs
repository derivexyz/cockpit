//! Forward variance on a piecewise-constant clock with scheduled events.
//! Times are UTC epoch seconds. Total variance is sigma squared times years.

use super::clock::VarClock;
use crate::SEC_PER_YEAR;
use anyhow::{ensure, Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// A scheduled release of variance.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Event {
    /// UTC epoch second at which the release starts.
    pub start_sec: i64,
    /// Duration of the linear fade in seconds, or zero for a jump.
    #[serde(default)]
    pub length_sec: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FwdVarConfig<C> {
    /// Supplies the variance time between expiries.
    pub clock: C,
    /// Scheduled events that can carry variance above the base rate.
    #[serde(default)]
    pub events: Vec<Event>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ForwardVarModel<C> {
    cfg: FwdVarConfig<C>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FitFlag {
    /// An event starts after the last expiry.
    EventDropped,
    /// The donor's base uses all the available variance or more.
    NegativeLeftover,
    /// Every interval contains an event.
    NoCleanDonor,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ForwardVarCurveFit {
    /// The term structure with its base rates and event sizes.
    pub curve: ForwardVarCurve,
    /// Distinct warnings raised during the fit.
    pub flags: Vec<FitFlag>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FittedEvent {
    /// UTC epoch second at which the release starts.
    pub start_sec: i64,
    /// Duration of the linear fade in seconds, or zero for a jump.
    pub length_sec: i64,
    /// Total variance released by this event.
    pub total_var: f64,
}

/// A self-contained fitted curve in wall time.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ForwardVarCurve {
    /// UTC epoch second from which the input variances were measured.
    pub ref_sec: i64,
    /// Last fitted expiry, in UTC epoch seconds.
    pub end_sec: i64,
    /// Base variance per second from each start until the next start or the last expiry.
    pub rates: Vec<(i64, f64)>,
    /// Events carrying positive variance in addition to the base rates.
    pub events: Vec<FittedEvent>,
}

/// Closest non-decreasing term structure by equal-weight least squares.
pub fn project_total_vars(total_vars: &BTreeMap<i64, f64>) -> BTreeMap<i64, f64> {
    // pool adjacent decreasing blocks, retaining their means and counts
    let mut blocks: Vec<(f64, usize)> = Vec::new();
    for &value in total_vars.values() {
        let (mut mean, mut count) = (value, 1);
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
    let values = blocks.into_iter().flat_map(|(mean, count)| std::iter::repeat_n(mean, count));
    total_vars.keys().copied().zip(values).collect()
}

fn spans(rates: &[(i64, f64)], end_sec: i64) -> impl Iterator<Item = (i64, i64, f64)> + '_ {
    let ends = rates.iter().skip(1).map(|p| p.0).chain([end_sec]);
    rates.iter().zip(ends).map(|(&(start, rate), end)| (start, end, rate))
}

fn check_window(start_sec: i64, length_sec: i64) -> Result<()> {
    ensure!(
        length_sec >= 0 && start_sec.checked_add(length_sec).is_some(),
        "invalid event window at {start_sec}"
    );
    Ok(())
}

fn validate_rates(rates: &[(i64, f64)], start: i64, end: i64) -> Result<()> {
    ensure!(
        rates.first().map(|p| p.0) == Some(start),
        "missing initial rate at {start}"
    );
    for (a, b, rate) in spans(rates, end) {
        ensure!(a < b && rate.is_finite() && rate >= 0., "invalid rate piece at {a}");
    }
    Ok(())
}

/// The stretch from the previous expiry to this one.
struct Interval {
    expiry: i64,
    weights: Vec<(i64, f64)>,
    /// variance added over the interval, and its clock mass
    added: f64,
    mass: f64,
    events: Vec<Event>,
}

impl Interval {
    /// Base level per unit of clock mass, and the variance of each of its events.
    fn allocate(&self, i: usize, donors: &[(usize, f64)], flags: &mut Vec<FitFlag>) -> (f64, f64) {
        let own = self.added / self.mass;
        if self.events.is_empty() {
            return (own, 0.);
        }
        // the nearest interval without an event lends its level, the later one on a tie
        let Some(&(_, donor)) = donors.iter().min_by_key(|&&(j, _)| (i.abs_diff(j), std::cmp::Reverse(j))) else {
            flag(flags, FitFlag::NoCleanDonor);
            return (own, 0.);
        };
        let leftover = self.added - donor * self.mass;
        // equal levels leave only rounding noise, of either sign
        if leftover <= 1e-9 * self.added {
            flag(flags, FitFlag::NegativeLeftover);
            return (own, 0.);
        }
        (donor, leftover / self.events.len() as f64)
    }
}

fn flag(flags: &mut Vec<FitFlag>, warning: FitFlag) {
    if !flags.contains(&warning) {
        flags.push(warning);
    }
}

impl<C: VarClock> ForwardVarModel<C> {
    pub fn new(cfg: FwdVarConfig<C>) -> Result<Self> {
        for e in &cfg.events {
            check_window(e.start_sec, e.length_sec)?;
        }
        Ok(Self { cfg })
    }

    pub fn config(&self) -> &FwdVarConfig<C> {
        &self.cfg
    }

    /// `total_vars` is expiry (sec) -> ATM total variance from `ref_sec`.
    pub fn fit(&self, ref_sec: i64, total_vars: &BTreeMap<i64, f64>) -> Result<ForwardVarCurveFit> {
        let (&end_sec, _) = total_vars.last_key_value().context("need at least one expiry")?;
        let (mut start, mut previous) = (ref_sec, 0.);
        let mut intervals = Vec::with_capacity(total_vars.len());
        for (&expiry, &total) in total_vars {
            ensure!(
                expiry > start && total.is_finite() && total > 0.,
                "need positive finite variance at future expiry {expiry}"
            );
            ensure!(total >= previous, "negative forward variance into expiry {expiry}");
            let weights = self.cfg.clock.weights(start, expiry)?;
            validate_rates(&weights, start, expiry)?;
            let mass: f64 = spans(&weights, expiry).map(|(a, b, weight)| b.abs_diff(a) as f64 * weight).sum();
            let added = total - previous;
            ensure!(
                (added / mass).is_finite(),
                "no clock mass or level into expiry {expiry}"
            );
            intervals.push(Interval {
                expiry,
                weights,
                added,
                mass,
                events: Vec::new(),
            });
            (start, previous) = (expiry, total);
        }

        let mut flags = Vec::new();
        for &e in &self.cfg.events {
            let end = e.start_sec + e.length_sec; // checked in new
            if end <= ref_sec {
                continue;
            }
            let i = intervals.partition_point(|interval| interval.expiry < e.start_sec);
            if let Some(interval) = intervals.get_mut(i) {
                ensure!(
                    e.start_sec > ref_sec && end <= interval.expiry,
                    "event at {} straddles ref_sec or an expiry",
                    e.start_sec
                );
                interval.events.push(e);
            } else {
                flag(&mut flags, FitFlag::EventDropped);
            }
        }

        let donors: Vec<_> = intervals
            .iter()
            .enumerate()
            .filter(|(_, v)| v.events.is_empty())
            .map(|(i, v)| (i, v.added / v.mass))
            .collect();
        let mut curve = ForwardVarCurve {
            ref_sec,
            end_sec,
            rates: Vec::new(),
            events: Vec::new(),
        };
        let mut rebuilt = 0.;
        for ((i, interval), &total) in intervals.iter().enumerate().zip(total_vars.values()) {
            let (level, total_var) = interval.allocate(i, &donors, &mut flags);
            for (start, end, weight) in spans(&interval.weights, interval.expiry) {
                curve.rates.push((start, level * weight));
                rebuilt += end.abs_diff(start) as f64 * (level * weight);
            }
            if total_var > 0. {
                for e in &interval.events {
                    curve.events.push(FittedEvent {
                        start_sec: e.start_sec,
                        length_sec: e.length_sec,
                        total_var,
                    });
                    rebuilt += total_var;
                }
            }
            // rounding stays far inside this; a rate lost to underflow or overflow does not
            ensure!(
                (rebuilt - total).abs() <= 1e-9 * total,
                "the curve cannot give back the variance at {}",
                interval.expiry
            );
        }
        Ok(ForwardVarCurveFit { curve, flags })
    }
}

impl ForwardVarCurve {
    fn validate(&self) -> Result<()> {
        validate_rates(&self.rates, self.ref_sec, self.end_sec)?;
        for e in &self.events {
            ensure!(e.total_var.is_finite() && e.total_var >= 0., "invalid event variance");
            check_window(e.start_sec, e.length_sec)?;
            ensure!(
                self.ref_sec < e.start_sec && e.start_sec + e.length_sec <= self.end_sec,
                "event at {} outside the curve",
                e.start_sec
            );
        }
        Ok(())
    }

    /// Total variance over (t_sec, expiry_sec], including a jump at the expiry.
    pub fn get_total_var(&self, t_sec: i64, expiry_sec: i64) -> Result<f64> {
        ensure!(
            self.ref_sec <= t_sec && t_sec <= expiry_sec && expiry_sec <= self.end_sec,
            "invalid query interval"
        );
        self.validate()?;
        let base = spans(&self.rates, self.end_sec).filter_map(|(start, end, rate)| {
            let (a, b) = (start.max(t_sec), end.min(expiry_sec));
            (a < b).then(|| b.abs_diff(a) as f64 * rate)
        });
        let releases = self.events.iter().map(|e| e.variance_between(t_sec, expiry_sec));
        let total = base.chain(releases).fold(0., |sum, v| sum + v);
        ensure!(total.is_finite(), "unrepresentable total variance");
        Ok(total.max(0.))
    }

    /// Right-continuous annualised variance rate at t_sec.
    pub fn get_forward_var(&self, t_sec: i64) -> Result<f64> {
        ensure!(
            (self.ref_sec..self.end_sec).contains(&t_sec),
            "{t_sec} is outside the fitted range"
        );
        self.validate()?;
        let i = self.rates.partition_point(|p| p.0 <= t_sec);
        let &(_, base) = self.rates[..i].last().context("missing base rate")?;
        // a fade releases at twice its average rate at the start, falling linearly to zero
        let fades = self.events.iter().filter_map(|e| {
            let (end, length) = (e.start_sec + e.length_sec, e.length_sec as f64);
            (e.start_sec..end)
                .contains(&t_sec)
                .then(|| 2. * e.total_var * ((end - t_sec) as f64 / length) / length)
        });
        let rate = (base + fades.sum::<f64>()) * SEC_PER_YEAR;
        ensure!(rate.is_finite(), "unrepresentable forward variance");
        Ok(rate)
    }
}

impl FittedEvent {
    fn variance_between(&self, from: i64, to: i64) -> f64 {
        if self.length_sec == 0 {
            return if from < self.start_sec && self.start_sec <= to {
                self.total_var
            } else {
                0.
            };
        }
        let end = self.start_sec + self.length_sec;
        let (a, b) = (from.max(self.start_sec), to.min(end));
        if a >= b {
            return 0.;
        }
        // trapezoid under the fading rate, even for a short query near the end
        let length = self.length_sec as f64;
        let share = (b - a) as f64 / length;
        let remaining = (end - a) as f64 / length + (end - b) as f64 / length;
        self.total_var * (share * remaining).min(1.)
    }
}
