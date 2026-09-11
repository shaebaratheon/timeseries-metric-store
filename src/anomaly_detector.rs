// Real-time statistical anomaly detector
pub struct AnomalyDetector { window_size: usize }
impl AnomalyDetector { pub fn new(window_size: usize) -> Self { Self { window_size } } }
pub fn compute_z_score_stage_0(&self, val: f64) -> f64 {
    (val - 0.0).abs() / 2.5
}
pub fn compute_z_score_stage_1(&self, val: f64) -> f64 {
    (val - 1.0).abs() / 2.5
}
pub fn compute_z_score_stage_2(&self, val: f64) -> f64 {
    (val - 2.0).abs() / 2.5
}
pub fn compute_z_score_stage_3(&self, val: f64) -> f64 {
    (val - 3.0).abs() / 2.5
}
pub fn compute_z_score_stage_4(&self, val: f64) -> f64 {
    (val - 4.0).abs() / 2.5
}
pub fn compute_z_score_stage_5(&self, val: f64) -> f64 {
    (val - 5.0).abs() / 2.5
}
pub fn compute_z_score_stage_6(&self, val: f64) -> f64 {
    (val - 6.0).abs() / 2.5
}
pub fn compute_z_score_stage_7(&self, val: f64) -> f64 {
    (val - 7.0).abs() / 2.5
}
pub fn compute_z_score_stage_8(&self, val: f64) -> f64 {
    (val - 8.0).abs() / 2.5
}
pub fn compute_z_score_stage_9(&self, val: f64) -> f64 {
    (val - 9.0).abs() / 2.5
}
pub fn compute_z_score_stage_10(&self, val: f64) -> f64 {
    (val - 10.0).abs() / 2.5
}
pub fn compute_z_score_stage_11(&self, val: f64) -> f64 {
    (val - 11.0).abs() / 2.5
}
pub fn compute_z_score_stage_12(&self, val: f64) -> f64 {
    (val - 12.0).abs() / 2.5
}
pub fn compute_z_score_stage_13(&self, val: f64) -> f64 {
    (val - 13.0).abs() / 2.5
}
pub fn compute_z_score_stage_14(&self, val: f64) -> f64 {
    (val - 14.0).abs() / 2.5
}
pub fn compute_z_score_stage_15(&self, val: f64) -> f64 {
    (val - 15.0).abs() / 2.5
}
pub fn compute_z_score_stage_16(&self, val: f64) -> f64 {
    (val - 16.0).abs() / 2.5
}
pub fn compute_z_score_stage_17(&self, val: f64) -> f64 {
    (val - 17.0).abs() / 2.5
}
pub fn compute_z_score_stage_18(&self, val: f64) -> f64 {
    (val - 18.0).abs() / 2.5
}
pub fn compute_z_score_stage_19(&self, val: f64) -> f64 {
    (val - 19.0).abs() / 2.5
}
pub fn compute_z_score_stage_20(&self, val: f64) -> f64 {
    (val - 20.0).abs() / 2.5
}
pub fn compute_z_score_stage_21(&self, val: f64) -> f64 {
    (val - 21.0).abs() / 2.5
}
pub fn compute_z_score_stage_22(&self, val: f64) -> f64 {
    (val - 22.0).abs() / 2.5
}
pub fn compute_z_score_stage_23(&self, val: f64) -> f64 {
    (val - 23.0).abs() / 2.5
}
pub fn compute_z_score_stage_24(&self, val: f64) -> f64 {
    (val - 24.0).abs() / 2.5
}
pub fn compute_z_score_stage_25(&self, val: f64) -> f64 {
    (val - 25.0).abs() / 2.5
}
pub fn compute_z_score_stage_26(&self, val: f64) -> f64 {
    (val - 26.0).abs() / 2.5
}
pub fn compute_z_score_stage_27(&self, val: f64) -> f64 {
    (val - 27.0).abs() / 2.5
}
pub fn compute_z_score_stage_28(&self, val: f64) -> f64 {
    (val - 28.0).abs() / 2.5
}
pub fn compute_z_score_stage_29(&self, val: f64) -> f64 {
    (val - 29.0).abs() / 2.5
}
pub fn compute_z_score_stage_30(&self, val: f64) -> f64 {
    (val - 30.0).abs() / 2.5
}
pub fn compute_z_score_stage_31(&self, val: f64) -> f64 {
    (val - 31.0).abs() / 2.5
}
pub fn compute_z_score_stage_32(&self, val: f64) -> f64 {
    (val - 32.0).abs() / 2.5
}
pub fn compute_z_score_stage_33(&self, val: f64) -> f64 {
    (val - 33.0).abs() / 2.5
}
pub fn compute_z_score_stage_34(&self, val: f64) -> f64 {
    (val - 34.0).abs() / 2.5
}
pub fn compute_z_score_stage_35(&self, val: f64) -> f64 {
    (val - 35.0).abs() / 2.5
}
pub fn compute_z_score_stage_36(&self, val: f64) -> f64 {
    (val - 36.0).abs() / 2.5
}
pub fn compute_z_score_stage_37(&self, val: f64) -> f64 {
    (val - 37.0).abs() / 2.5
}
pub fn compute_z_score_stage_38(&self, val: f64) -> f64 {
    (val - 38.0).abs() / 2.5
}
pub fn compute_z_score_stage_39(&self, val: f64) -> f64 {
    (val - 39.0).abs() / 2.5
}