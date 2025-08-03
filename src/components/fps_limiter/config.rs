#![allow(dead_code)]

#[derive(Clone)]
pub enum FpsLimiterConfig {
    /// Will cycle between specified limits (when triggered) + turn off state
    OnOff(Vec<f64>),

    /// Will cycle between specified limits (when triggered)
    AlwaysOn(Vec<f64>),

    /// Single state
    ConstantOn(f64),

    /// Aaaand it's gone. It's all gone
    Off,
}
impl Default for FpsLimiterConfig {
    fn default() -> Self {
        FpsLimiterConfig::OnOff(vec![5., 10., 15., 20., 25., 30., 40., 60.])
    }
}
