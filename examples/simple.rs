use gewy::kurbo::RoundedRectRadii;
use gewy::peniko::Color;
use gewy::{div, div_begin, end, margin_all, pc, px, size, size_all, text, Div, FontDB, GewyApp, GewyAppEvent, GewyContext, GewyWindow, Renderer, Text, TextAlign, Widget};
use gewy::taffy::*;

fn main() {
    env_logger::init();
    let font_db = FontDB::load("assets/fonts/Roboto-Regular.ttf").unwrap();
    let app = GewyApp::new(font_db);
    app.start(|event, ctx| {
        match event {
            GewyAppEvent::Start => on_start(ctx),
            GewyAppEvent::Stop  => on_stop(),
        }
    });
}

fn on_start(mut ctx: GewyContext) {
    ctx.add_window(GewyWindow::new(512, 512, AppWidget));
}

fn on_stop() {
    println!("GewyApp finished running!");
}

struct AppWidget;
impl Widget for AppWidget {

    fn style(&self, s: &mut Style) {
        s.size.width = pc(1.0);
        s.size.height = pc(1.0);
        s.flex_direction = FlexDirection::Column;
        s.justify_content = Some(JustifyContent::Center);
        s.align_items = Some(AlignItems::Center);
    }

    fn render(&self, r: &mut Renderer) {
        div(red, r);
        div_begin(gray, r);
            div(green, r);
            text("This is some text!", txt, r);
            div(yellow, r);
        end(r);
        div(blue, r);
    }
}


// --------------- Classes --------------- 

fn txt(t: &mut Text) {
    t.color = Color::BLACK;
    t.text_align = TextAlign::Center;
}

fn red(d: &mut Div) {
    d.radii = RoundedRectRadii::from_single_radius(5.0);
    d.style.size = size_all(px(32));
    d.color = Color::RED;
}

fn yellow(d: &mut Div) {
    let s = &mut d.style;
    d.radii = RoundedRectRadii::from_single_radius(5.0);
    d.color = Color::YELLOW;
    s.size = size_all(px(64));
    s.margin = margin_all(px(5));
    s.size = size_all(px(32));
}

fn green(d: &mut Div) {
    d.radii = RoundedRectRadii::from_single_radius(5.0);
    d.style.size = size_all(px(32));
    d.color = Color::GREEN;
    d.style.margin = margin_all(px(5));
}

fn blue(d: &mut Div) {
    d.radii = RoundedRectRadii::from_single_radius(5.0);
    d.style.size = size_all(px(32));
    d.color = Color::BLUE;
}

fn gray(d: &mut Div) {
    let s = &mut d.style;
    d.radii = RoundedRectRadii::from_single_radius(20.0);
    d.color = Color::GRAY;
    s.flex_direction = FlexDirection::Column;
    s.justify_content = Some(JustifyContent::Center);
    s.align_items = Some(AlignItems::Center);
    s.flex_direction = FlexDirection::Column;
    s.size = size(pc(0.8), px(256));
}