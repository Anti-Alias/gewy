use gewy::geom::RoundedRectRadii;
use gewy::paint::Color;
use gewy::{begin, div, end, Div, GewyApp, GewyWindowState, UIRenderer, Widget};
use gewy::layout::*;

fn main() {
    env_logger::init();
    let mut app = GewyApp::new();
    app.add_window(GewyWindowState::new(512, 512, AppWidget));
    app.start();
}


struct AppWidget;
impl Widget for AppWidget {

    fn style(&self) -> Style {
        let mut style = Style::default();
        style.size.width = Dimension::Percent(1.0);
        style.size.height = Dimension::Percent(1.0);
        style.flex_direction = FlexDirection::Column;
        style.justify_content = Some(JustifyContent::SpaceAround);
        style.align_items = Some(AlignItems::Center);
        style
    }

    fn render(&self, r: &mut UIRenderer) {
        div((c_round, c_red), r);
        div(c_gray, r); begin(r);
            div((c_round, c_green), r);
            div((c_round, c_yellow), r);
        end(r);
        div((c_round, c_blue), r);
    }
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
}

fn c_blue(d: &mut Div) {
    d.color = Color::BLUE;
}

fn c_gray(d: &mut Div) {
    let s = &mut d.style;
    s.flex_direction = FlexDirection::Column;
    s.justify_content = Some(JustifyContent::Center);
    s.align_items = Some(AlignItems::Center);
    d.radii = RoundedRectRadii::from_single_radius(5.0);
    d.style.size = Size::from_lengths(128.0, 129.0);
    d.color = Color::GRAY;
}