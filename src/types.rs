use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, PartialEq)]
pub struct MetricPoint {
    pub timestamp_ms: u64,
    pub value: f64,
}

impl MetricPoint {
    pub fn now(value: f64) -> Self {
        let ts = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64;
        Self { timestamp_ms: ts, value }
    }

    pub fn new(timestamp_ms: u64, value: f64) -> Self {
        Self { timestamp_ms, value }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AggregationWindow {
    OneSecond = 1_000,
    OneMinute = 60_000,
    FiveMinutes = 300_000,
    OneHour = 3_600_000,
}

#[derive(Debug, Clone)]
pub struct AggregatedBucket {
    pub window_start_ms: u64,
    pub count: usize,
    pub sum: f64,
    pub min: f64,
    pub max: f64,
}

impl AggregatedBucket {
    pub fn new(window_start_ms: u64) -> Self {
        Self {
            window_start_ms,
            count: 0,
            sum: 0.0,
            min: f64::INFINITY,
            max: f64::NEG_INFINITY,
        }
    }

    pub fn record(&mut self, val: f64) {
        self.count += 1;
        self.sum += val;
        if val < self.min { self.min = val; }
        if val > self.max { self.max = val; }
    }

    pub fn avg(&self) -> f64 {
        if self.count == 0 { 0.0 } else { self.sum / (self.count as f64) }
    }
}

// Metric data type definitions

// Metric data type definitions

// Metric data type definitions

// Metric data type definitions

// Metric data type definitions

// Metric data type definitions

// Metric data type definitions

// Metric data type definitions

// Metric data type definitions

// Metric data type definitions

// Metric data type definitions

// Metric data type definitions

// Metric data type definitions

// Metric data type definitions

// Metric data type definitions

// Metric data type definitions

// Metric data type definitions

// Metric data type definitions

// Metric data type definitions

// Metric data type definitions

// Metric data type definitions

// Metric data type definitions

// Metric data type definitions

// Metric data type definitions

// Metric data type definitions

// Metric data type definitions

// Metric data type definitions

// Metric data type definitions

// Metric data type definitions

// Metric data type definitions

// Metric data type definitions

// Metric data type definitions

// Metric data type definitions

// Metric data type definitions

// Metric data type definitions

// Metric data type definitions

// Metric data type definitions

// Metric data type definitions

// Metric data type definitions

// Metric data type definitions
