#![allow(dead_code)]

use bevy::prelude::*;
use bevy::{
    color::palettes::css::GOLD,
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
};

use super::*;

pub fn setup(mut commands: Commands) {
    commands
        .spawn((Text::new("FPS: "), markers::FpsCounterText))
        .with_child((
            TextSpan::default(),
            TextColor(GOLD.into()),
            markers::FpsCounterValueText,
        ));
}

pub fn update_fps_value(
    diagnostics: Res<DiagnosticsStore>,
    mut query: Query<(&mut TextSpan, &mut TextColor), With<markers::FpsCounterValueText>>,
) {
    for (mut text, mut color) in &mut query {
        if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(value) = fps.smoothed() {
                // Update the value of the second section
                **text = format!("{value:.2}");
                color.0 = fps_to_color(value as f32);
            }
        }
    }
}

// TODO: refactor for better understanding
// GPT code
fn fps_to_color(fps: f32) -> Color {
    // Clamp FPS to reasonable range (0-500)
    let fps = fps.clamp(0.0, 500.0);

    // Define our transition points (FPS value, Color)
    let color_stops = [
        (10.0, Color::srgb(1.0, 0.0, 0.0)), // Red at 10 FPS
        (40.0, Color::srgb(1.0, 0.5, 0.0)), // Orange at 40 FPS
        (60.0, Color::srgb(0.0, 1.0, 0.0)), // Green at 60 FPS
    ];

    // Find which segment we're in
    if fps <= color_stops[0].0 {
        return color_stops[0].1; // Below 10 - pure red
    } else if fps >= color_stops[2].0 {
        return color_stops[2].1; // Above 60 - pure green
    }

    // Find the segment we're between
    let segment = if fps <= color_stops[1].0 {
        // Between red (10) and orange (40)
        (&color_stops[0], &color_stops[1])
    } else {
        // Between orange (40) and green (60)
        (&color_stops[1], &color_stops[2])
    };

    // Calculate interpolation factor (0.0 to 1.0)
    let t = (fps - segment.0.0) / (segment.1.0 - segment.0.0);

    segment.0.1.to_srgba().red;

    // Linearly interpolate between the two colors
    Color::srgb(
        segment.0.1.to_srgba().red + t * (segment.1.1.to_srgba().red - segment.0.1.to_srgba().red),
        segment.0.1.to_srgba().green
            + t * (segment.1.1.to_srgba().green - segment.0.1.to_srgba().green),
        segment.0.1.to_srgba().blue
            + t * (segment.1.1.to_srgba().blue - segment.0.1.to_srgba().blue),
    )
}
