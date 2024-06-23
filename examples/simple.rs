use gewy::kurbo::RoundedRectRadii;
use gewy::peniko::Color;
use gewy::{div, div_begin, end, margin_all, pc, px, size, size_all, text, App, Div, FontDB, Text, TextAlign, View, Wid, Window};
use gewy::taffy::*;

fn main() {
    env_logger::init();
    let font_db = FontDB::load("assets/fonts/Roboto-Regular.ttf").unwrap();
    App::new(font_db).start(|ctx| {
        let app_widget = Wid::new(app_view, app_c);
        ctx.add_window(Window::new(512, 512, app_widget));
    });
}

// View function that produces a widget tree.
fn app_view(v: &mut View) {
    div(red_c, v);
    div_begin(gray_c, v);
        div(green_c, v);
        text("This is some text!", text_c, v);
        div(yellow_c, v);
    end(v);
    div(blue_c, v);
}


// "Classes" are just callback functions that externalize widget configuration
fn app_c(s: &mut Style) {
    s.size.width = pc(1.0);
    s.size.height = pc(1.0);
    s.flex_direction = FlexDirection::Column;
    s.justify_content = Some(JustifyContent::Center);
    s.align_items = Some(AlignItems::Center);
}

fn text_c(t: &mut Text) {
    t.color = Color::BLACK;
    t.text_align = TextAlign::Center;
}

fn red_c(d: &mut Div) {
    d.radii = RoundedRectRadii::from_single_radius(5.0);
    d.style.size = size_all(px(32));
    d.color = Color::RED;
}

fn yellow_c(d: &mut Div) {
    let s = &mut d.style;
    d.radii = RoundedRectRadii::from_single_radius(5.0);
    d.color = Color::YELLOW;
    s.size = size_all(px(64));
    s.margin = margin_all(px(5));
    s.size = size_all(px(32));
}

fn green_c(d: &mut Div) {
    d.radii = RoundedRectRadii::from_single_radius(5.0);
    d.style.size = size_all(px(32));
    d.color = Color::GREEN;
    d.style.margin = margin_all(px(5));
}

fn blue_c(d: &mut Div) {
    d.radii = RoundedRectRadii::from_single_radius(5.0);
    d.style.size = size_all(px(32));
    d.color = Color::BLUE;
}

fn gray_c(d: &mut Div) {
    let s = &mut d.style;
    d.radii = RoundedRectRadii::from_single_radius(20.0);
    d.color = Color::GRAY;
    s.flex_direction = FlexDirection::Column;
    s.justify_content = Some(JustifyContent::Center);
    s.align_items = Some(AlignItems::Center);
    s.flex_direction = FlexDirection::Column;
    s.size = size(pc(0.8), px(256));
}