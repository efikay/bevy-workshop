#[derive(Default)]
pub enum ZLevel {
    /// For floor tiles only (no obstacles)
    Floor,

    /// For creatures, obstacles, walls (basic level)
    #[default]
    Ground,
}

impl Into<f32> for ZLevel {
    fn into(self) -> f32 {
        match self {
            ZLevel::Floor => -1.0,
            ZLevel::Ground => 0.0,
        }
    }
}
