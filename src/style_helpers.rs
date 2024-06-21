
pub mod margin {
    use crate::taffy::{Rect, LengthPercentageAuto};

    pub fn px(top: f32, right: f32, bottom: f32, left: f32) -> Rect<LengthPercentageAuto> {
        Rect {
            left: LengthPercentageAuto::Length(left),
            right: LengthPercentageAuto::Length(right),
            top: LengthPercentageAuto::Length(top),
            bottom: LengthPercentageAuto::Length(bottom),
        }
    }

    pub fn pc(top: f32, right: f32, bottom: f32, left: f32) -> Rect<LengthPercentageAuto> {
        Rect {
            left: LengthPercentageAuto::Percent(left),
            right: LengthPercentageAuto::Percent(right),
            top: LengthPercentageAuto::Percent(top),
            bottom: LengthPercentageAuto::Percent(bottom),
        }
    }

    pub fn px_all(value: f32) -> Rect<LengthPercentageAuto> {
        Rect {
            left: LengthPercentageAuto::Length(value),
            right: LengthPercentageAuto::Length(value),
            top: LengthPercentageAuto::Length(value),
            bottom: LengthPercentageAuto::Length(value),
        }
    }
}

pub mod padding {

    use crate::taffy::{Rect, LengthPercentage};

    pub fn px(top: f32, right: f32, bottom: f32, left: f32) -> Rect<LengthPercentage> {
        Rect {
            left: LengthPercentage::Length(left),
            right: LengthPercentage::Length(right),
            top: LengthPercentage::Length(top),
            bottom: LengthPercentage::Length(bottom),
        }
    }

    pub fn pc(top: f32, right: f32, bottom: f32, left: f32) -> Rect<LengthPercentage> {
        Rect {
            left: LengthPercentage::Percent(left),
            right: LengthPercentage::Percent(right),
            top: LengthPercentage::Percent(top),
            bottom: LengthPercentage::Percent(bottom),
        }
    }

    pub fn px_all(value: f32) -> Rect<LengthPercentage> {
        Rect {
            left: LengthPercentage::Length(value),
            right: LengthPercentage::Length(value),
            top: LengthPercentage::Length(value),
            bottom: LengthPercentage::Length(value),
        }
    }
}

pub mod size {
    use crate::taffy::{Size, Dimension};

    pub fn px(width: f32, height: f32) -> Size<Dimension> {
        Size {
            width: Dimension::Length(width),
            height: Dimension::Length(height),
        }
    }

    pub fn pc(width: f32, height: f32) -> Size<Dimension> {
        Size {
            width: Dimension::Percent(width),
            height: Dimension::Percent(height),
        }
    }

    pub fn width_px(width: f32) -> Size<Dimension> {
        Size {
            width: Dimension::Length(width),
            height: Dimension::Auto,
        }
    }

    pub fn width_pc(width: f32) -> Size<Dimension> {
        Size {
            width: Dimension::Percent(width),
            height: Dimension::Auto,
        }
    }
}

pub mod radii {
    use crate::taffy::{Size, Dimension};
    
    pub fn px(width: f32, height: f32) -> Size<Dimension> {
        Size {
            width: Dimension::Length(width),
            height: Dimension::Length(height),
        }
    }

    pub fn pc(width: f32, height: f32) -> Size<Dimension> {
        Size {
            width: Dimension::Percent(width),
            height: Dimension::Percent(height),
        }
    }
}