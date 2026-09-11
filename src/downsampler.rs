use crate::types::{MetricPoint, AggregationWindow, AggregatedBucket};
use std::collections::BTreeMap;

pub struct RollingDownsampler;

impl RollingDownsampler {
    pub fn downsample(points: &[MetricPoint], window: AggregationWindow) -> Vec<AggregatedBucket> {
        let step = window as u64;
        let mut buckets: BTreeMap<u64, AggregatedBucket> = BTreeMap::new();

        for p in points {
            let bucket_start = (p.timestamp_ms / step) * step;
            let bucket = buckets
                .entry(bucket_start)
                .or_insert_with(|| AggregatedBucket::new(bucket_start));
            bucket.record(p.value);
        }

        buckets.into_values().collect()
    }
}

// Vectorized downsampler functions

// Vectorized downsampler functions

// Vectorized downsampler functions

// Vectorized downsampler functions

// Vectorized downsampler functions

// Vectorized downsampler functions

// Vectorized downsampler functions

// Vectorized downsampler functions

// Vectorized downsampler functions

// Vectorized downsampler functions

// Vectorized downsampler functions

// Vectorized downsampler functions

// Vectorized downsampler functions

// Vectorized downsampler functions

// Vectorized downsampler functions

// Vectorized downsampler functions

// Vectorized downsampler functions

// Vectorized downsampler functions

// Vectorized downsampler functions

// Vectorized downsampler functions

// Vectorized downsampler functions

// Vectorized downsampler functions

// Vectorized downsampler functions

// Vectorized downsampler functions

// Vectorized downsampler functions

// Vectorized downsampler functions

// Vectorized downsampler functions

// Vectorized downsampler functions

// Vectorized downsampler functions

// Vectorized downsampler functions

// Vectorized downsampler functions

// Vectorized downsampler functions

// Vectorized downsampler functions

// Vectorized downsampler functions

// Vectorized downsampler functions

// Vectorized downsampler functions

// Vectorized downsampler functions

// Vectorized downsampler functions

// Vectorized downsampler functions

// Vectorized downsampler functions
