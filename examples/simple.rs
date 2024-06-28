use gewy::kurbo::RoundedRectRadii;
use gewy::peniko::Color;
use gewy::{div, div_begin, end, margin_all, pc, px, size, size_all, text, App, Div, FontDB, Text, TextAlign, View, Wid, Window};
use gewy::taffy::*;

fn main() {
    env_logger::init();
    let fonts = FontDB::load("assets/fonts/Roboto-Regular.ttf").unwrap();
    App::new(fonts).start(|ctx| {
        let widget = Wid::new(simple_fn, simple_cls);
        ctx.add_window(Window::new(512, 512, widget));
    });
}

// View function
fn simple_fn(v: &mut View) {
    div(red_cls, v);
    div_begin(gray_cls, v);
        div(green_cls, v);
        text("This is some text!", text_cls, v);
        div(yellow_cls, v);
    end(v);
    div(blue_cls, v);
}


// ---------- Class functions ----------
fn simple_cls(s: &mut Style) {
    s.size.width = pc(1.0);
    s.size.height = pc(1.0);
    s.flex_direction = FlexDirection::Column;
    s.justify_content = Some(JustifyContent::Center);
    s.align_items = Some(AlignItems::Center);
}

fn text_cls(t: &mut Text) {
    t.color = Color::BLACK;
    t.text_align = TextAlign::Center;
}

fn red_cls(d: &mut Div) {
    d.radii = RoundedRectRadii::from_single_radius(5.0);
    d.style.size = size_all(px(32));
    d.color = Color::RED;
}

fn yellow_cls(d: &mut Div) {
    let s = &mut d.style;
    d.radii = RoundedRectRadii::from_single_radius(5.0);
    d.color = Color::YELLOW;
    s.size = size_all(px(64));
    s.margin = margin_all(px(5));
    s.size = size_all(px(32));
}

fn green_cls(d: &mut Div) {
    d.radii = RoundedRectRadii::from_single_radius(5.0);
    d.style.size = size_all(px(32));
    d.color = Color::GREEN;
    d.style.margin = margin_all(px(5));
}

fn blue_cls(d: &mut Div) {
    d.radii = RoundedRectRadii::from_single_radius(5.0);
    d.style.size = size_all(px(32));
    d.color = Color::BLUE;
}

fn gray_cls(d: &mut Div) {
    let s = &mut d.style;
    d.radii = RoundedRectRadii::from_single_radius(20.0);
    d.color = Color::GRAY;
    s.flex_direction = FlexDirection::Column;
    s.justify_content = Some(JustifyContent::Center);
    s.align_items = Some(AlignItems::Center);
    s.flex_direction = FlexDirection::Column;
    s.size = size(pc(0.8), px(256));
}