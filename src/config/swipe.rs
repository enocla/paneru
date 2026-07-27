use serde::Deserialize;

use crate::{config::deserialize_modifier, platform::Modifiers};

#[derive(Clone, Debug, Deserialize)]
pub enum SwipeGestureDirection {
    Natural,
    Reversed,
}

#[derive(Deserialize, Clone, Debug, Default)]
pub struct SwipeOptions {
    /// Swipe sensitivity multiplier. Lower values = less distance per finger
    /// movement. Range: 0.1–2.0. Default: 0.35.
    pub sensitivity: Option<f64>,

    /// Swipe inertia deceleration rate. Higher values = faster stop.
    /// Range: 1.0–10.0. Default: 4.0.
    pub deceleration: Option<f64>,

    /// Swiping keeps sliding windows until the first or last window.
    /// Set to false to clamp so edge windows stay on-screen. Default: true.
    #[allow(dead_code)]
    pub continuous: Option<bool>,

    /// Move focus one column at a time instead of sliding the strip.
    /// Applies to both trackpad gestures and modifier scrolling. Default: false.
    pub discrete: Option<bool>,

    /// Accumulated swipe distance required for one discrete focus step, before
    /// sensitivity scaling. Units match the raw gesture delta: fractions of
    /// trackpad travel summed across fingers, so three fingers each moving 5%
    /// of the trackpad contribute 0.15.
    /// Range: 0.01–2.0. Default: 0.15.
    pub discrete_threshold: Option<f64>,

    pub gesture: Option<GestureOptions>,
    pub scroll: Option<ScrollOptions>,
}

#[derive(Deserialize, Clone, Debug, Default)]
pub struct GestureOptions {
    /// The number of fingers required for swipe gestures to move windows.
    pub fingers_count: Option<usize>,

    /// Which direction swipe gestures should move windows.
    pub direction: Option<SwipeGestureDirection>,

    /// Whether to intercept vertical swipes.
    pub vertical: Option<bool>,
}

#[derive(Deserialize, Clone, Debug, Default)]
pub struct ScrollOptions {
    /// Modifier key(s) required for scroll wheel swiping.
    /// Accepts the same format as keybindings: "alt", "cmd", "alt + cmd", "alt + rcmd" etc.
    #[serde(default, deserialize_with = "deserialize_modifier")]
    pub modifier: Option<Modifiers>,

    /// Additional modifier key(s) that, combined with the scroll modifier,
    /// switches virtual workspaces vertically instead of scrolling horizontally.
    #[serde(default, deserialize_with = "deserialize_modifier")]
    pub vertical_modifier: Option<Modifiers>,
}
