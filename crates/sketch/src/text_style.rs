//! How a text is set (font style, alignment, spacing, path). Plain data, kept apart from
//! `crate::text` so sketches carrying text links still load and save when the `text` feature
//! (font loading and glyph layout) is off.

use serde::{Deserialize, Serialize};

/// Where each line of a text sits against its anchor (`at`, or a point of its path).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TextAlign {
    /// The line starts at the anchor.
    #[default]
    Left,
    /// The line is centred on it.
    Center,
    /// The line ends at it.
    Right,
}

impl TextAlign {
    fn is_left(&self) -> bool {
        *self == TextAlign::Left
    }
}

/// How a text is set: the style of its font, where its lines sit, the space between its letters
/// and lines, and the sketch curve its baseline follows (instead of the straight line from `at`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TextStyle {
    #[serde(default, skip_serializing_if = "is_false")]
    pub bold: bool,
    #[serde(default, skip_serializing_if = "is_false")]
    pub italic: bool,
    #[serde(default, skip_serializing_if = "TextAlign::is_left")]
    pub align: TextAlign,
    /// Extra space between letters (mm; negative brings them closer).
    #[serde(default, skip_serializing_if = "is_zero")]
    pub letter_spacing: f64,
    /// The distance between lines as a multiple of the font's own.
    #[serde(default = "one", skip_serializing_if = "is_one")]
    pub line_spacing: f64,
    /// The id of a curve of the same sketch the baseline follows (`at` and the angle then do not
    /// place the text).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// Along the path the other way round, so the text sits on its other side.
    #[serde(default, skip_serializing_if = "is_false")]
    pub reverse: bool,
}

impl Default for TextStyle {
    fn default() -> Self {
        TextStyle { bold: false, italic: false, align: TextAlign::Left, letter_spacing: 0.0, line_spacing: 1.0, path: None, reverse: false }
    }
}

impl TextStyle {
    /// Nothing but the defaults (what a text saved before styles had).
    pub fn is_plain(&self) -> bool {
        *self == TextStyle::default()
    }
}

fn is_false(b: &bool) -> bool {
    !*b
}

fn is_zero(x: &f64) -> bool {
    *x == 0.0
}

fn one() -> f64 {
    1.0
}

fn is_one(x: &f64) -> bool {
    *x == 1.0
}
