use std::path::Path;
use gewy::geom::RoundedRectRadii;
use gewy::paint::{Blob, Color, Font};
use gewy::{begin, div, end, text, Div, FontDB, GewyApp, GewyWindowState, Text, TextAlign, UIRenderer, Widget};
use gewy::layout::*;

fn main() {
    env_logger::init();

    // Loads fonts
    let default_font = load_font("assets/fonts/arial.ttf").unwrap();
    let font_db = FontDB::new(default_font);

    // Starts app
    let mut app = GewyApp::new(font_db);
    app.add_window(GewyWindowState::new(512, 512, AppWidget));
    app.start();
}

fn load_font(path: impl AsRef<Path>) -> Result<Font, std::io::Error> {
    let bytes = std::fs::read(path)?;
    let bytes = Blob::from(bytes);
    Ok(Font::new(bytes, 0))
}


struct AppWidget;
impl Widget for AppWidget {

    fn style(&self, s: &mut Style) {
        s.size.width = Dimension::Percent(1.0);
        s.size.height = Dimension::Percent(1.0);
        s.flex_direction = FlexDirection::Column;
        s.justify_content = Some(JustifyContent::Center);
        s.align_items = Some(AlignItems::Center);
    }

    fn render(&self, r: &mut UIRenderer) {
        text("This is some text!", c_text, r);
        div((c_round, c_red), r);
        div(c_gray, r); begin(r);
            div((c_round, c_green), r);
            div((c_round, c_yellow), r);
        end(r);
        div((c_round, c_blue), r);
    }
}

fn c_text(t: &mut Text) {
    t.width = Some(64.0);
}

fn c_round(d: &mut Div) {
    d.radii = RoundedRectRadii::from_single_radius(5.0);
    d.style.size = Size::from_lengths(32.0, 32.0);
}

fn c_red(d: &mut Div) {
    d.color = Color::RED;
}

fn c_yellow(d: &mut Div) {
    d.color = Color::YELLOW;
    d.style.margin = Rect {
        left: LengthPercentageAuto::Length(5.0),
        right: LengthPercentageAuto::Length(5.0),
        top: LengthPercentageAuto::Length(5.0),
        bottom: LengthPercentageAuto::Length(5.0),
    };
}

fn c_green(d: &mut Div) {
    d.color = Color::GREEN;
    d.style.margin = Rect {
        left: LengthPercentageAuto::Length(5.0),
        right: LengthPercentageAuto::Length(5.0),
        top: LengthPercentageAuto::Length(5.0),
        bottom: LengthPercentageAuto::Length(5.0),
    };
}

fn c_blue(d: &mut Div) {
    d.color = Color::BLUE;
}

fn c_gray(d: &mut Div) {
    let s = &mut d.style;
    s.flex_direction = FlexDirection::Column;
    s.justify_content = Some(JustifyContent::Center);
    s.align_items = Some(AlignItems::Center);
    d.radii = RoundedRectRadii::from_single_radius(20.0);
    d.style.size = Size::from_lengths(128.0, 129.0);
    d.color = Color::GRAY;
}