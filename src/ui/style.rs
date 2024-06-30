use taffy::{Dimension, JustifyContent, LengthPercentage, LengthPercentageAuto, Rect, Size, Style};

/// A suitable style for a root component.
/// Expands to the full size of its parent and centers its content.
pub fn root_style() -> Style {
    let mut style = Style::DEFAULT;
    style.size.width = Dimension::Percent(1.0);
    style.size.height = Dimension::Percent(1.0);
    style.justify_content = Some(JustifyContent::Center);
    style.align_items = Some(taffy::AlignItems::Center);
    style
}

/// A style helper function for pixel values.
/// Converts either into a [`Dimension`], a [`LengthPercentage`] or a [`LengthPercentageAuto`].
#[inline(always)]
pub fn px<R: From<Val>>(value: impl ToFloat) -> R {
    let float = value.to_f32();
    Val::Px(float).into()
}

/// A style helper function for percentage values.
/// Converts either into a [`Dimension`], a [`LengthPercentage`] or a [`LengthPercentageAuto`].
#[inline(always)]
pub fn pc<R: From<Val>>(value: f32) -> R {
    Val::Pc(value).into()
}

/// A style helper function for percentage values.
/// Converts either into a [`Dimension`], a [`LengthPercentage`] or a [`LengthPercentageAuto`].
#[inline(always)]
pub fn auto<R: From<Val>>() -> R {
    Val::Auto.into()
}

/// A style helper function for producing a margin rect.
#[inline(always)]
pub fn margin(top: Val, right: Val, bottom: Val, left: Val) -> Rect<LengthPercentageAuto> {
    Rect {
        top: top.into(),
        right: right.into(),
        bottom: bottom.into(),
        left: left.into(),
    }
}

/// A style helper function for producing a margin rect.
#[inline(always)]
pub fn margin_all(value: Val) -> Rect<LengthPercentageAuto> {
    Rect {
        top: value.into(),
        right: value.into(),
        bottom: value.into(),
        left: value.into(),
    }
}

/// A style helper function for producing a padding rect.
#[inline(always)]
pub fn padding(top: Val, right: Val, bottom: Val, left: Val) -> Rect<LengthPercentage> {
    Rect {
        top: top.into(),
        right: right.into(),
        bottom: bottom.into(),
        left: left.into(),
    }
}

/// A style helper function for producing a padding rect.
#[inline(always)]
pub fn padding_all(value: Val) -> Rect<LengthPercentage> {
    Rect {
        top: value.into(),
        right: value.into(),
        bottom: value.into(),
        left: value.into(),
    }
}

/// A style helper function for producing a size.
#[inline(always)]
pub fn size(width: Val, height: Val) -> Size<Dimension> {
    Size {
        width: width.into(),
        height: height.into(),
    }
}

/// A style helper function for producing a size.
#[inline(always)]
pub fn size_all(value: Val) -> Size<Dimension> {
    Size {
        width: value.into(),
        height: value.into(),
    }
}


/// A type that simplifies providing values for various [`Style`](crate::taffy::Style)s.
/// The unit for a val is either px (pixels), pc (percent) or auto.
#[derive(Copy, Clone, PartialEq, Default, Debug)]
pub enum Val {
    Px(f32),
    Pc(f32),
    #[default]
    Auto,
}

impl From<Val> for Dimension {
    fn from(val: Val) -> Self {
        match val {
            Val::Px(px) => Self::Length(px),
            Val::Pc(pc) => Self::Percent(pc),
            Val::Auto   => Self::Length(0.0),
        }
    }
}

impl From<Val> for LengthPercentageAuto {
    fn from(val: Val) -> Self {
        match val {
            Val::Px(px) => Self::Length(px),
            Val::Pc(pc) => Self::Percent(pc),
            Val::Auto => Self::Auto,
        }
    }
}

impl From<Val> for LengthPercentage {
    fn from(val: Val) -> Self {
        match val {
            Val::Px(px) => Self::Length(px),
            Val::Pc(pc) => Self::Percent(pc),
            Val::Auto   => Self::Length(0.0),
        }
    }
}


pub trait ToFloat {
    fn to_f32(self) -> f32;
}

impl ToFloat for f32 {
    fn to_f32(self) -> f32 { self }
}

impl ToFloat for f64 {
    fn to_f32(self) -> f32 { self as f32 }
}

impl ToFloat for i8 {
    fn to_f32(self) -> f32 { self as f32 }
}

impl ToFloat for i16 {
    fn to_f32(self) -> f32 { self as f32 }
}

impl ToFloat for i32 {
    fn to_f32(self) -> f32 { self as f32 }
}

impl ToFloat for i64 {
    fn to_f32(self) -> f32 { self as f32 }
}

impl ToFloat for i128 {
    fn to_f32(self) -> f32 { self as f32 }
}

impl ToFloat for isize {
    fn to_f32(self) -> f32 { self as f32 }
}

impl ToFloat for u8 {
    fn to_f32(self) -> f32 { self as f32 }
}

impl ToFloat for u16 {
    fn to_f32(self) -> f32 { self as f32 }
}

impl ToFloat for u32 {
    fn to_f32(self) -> f32 { self as f32 }
}

impl ToFloat for u64 {
    fn to_f32(self) -> f32 { self as f32 }
}

impl ToFloat for u128 {
    fn to_f32(self) -> f32 { self as f32 }
}

impl ToFloat for usize {
    fn to_f32(self) -> f32 { self as f32 }
}