use gewy::kurbo::RoundedRectRadii;
use gewy::peniko::Color;
use gewy::{div_begin, end, margin_all, padding_all, pc, px, size_all, text, Div, FontDB, GewyApp, GewyAppEvent, GewyContext, GewyWindow, Renderer, Text, ToGewyString, Widget};
use gewy::taffy::*;

fn main() {
    env_logger::init();
    let font_db = FontDB::load("assets/fonts/Roboto-Regular.ttf").unwrap();
    let app = GewyApp::new(font_db);
    app.start(|event, ctx| {
        match event {
            GewyAppEvent::Start => on_start(ctx),
            _ => {}
        }
    });
}

fn on_start(mut ctx: GewyContext) {
    ctx.add_window(GewyWindow::new(512, 512, AppWidget));
}


struct AppWidget;
impl Widget for AppWidget {

    fn style(&self, s: &mut Style) {
        s.size = size_all(pc(1.0));
        s.flex_direction = FlexDirection::Column;
        s.justify_content = Some(JustifyContent::Center);
        s.align_items = Some(AlignItems::Center);
    }

    fn render(&self, r: &mut Renderer) {
        counter(r);
        counter(r);
        counter(r);
    }
}

// --------------- Widget functions --------------- 
fn counter(r: &mut Renderer) {
    div_begin(c_counter, r);
        text("Count: 0", text_dark, r);
        div_begin(buttons, r);
            button_dark("+", r);
            button_dark("-", r);
        end(r);
    end(r);
}

fn button_dark(string: impl ToGewyString, r: &mut Renderer) {
    div_begin(inc_dec, r);
        text(string, text_light, r);
    end(r);
}


// --------------- Classes --------------- 
fn c_counter(d: &mut Div) {
    let s = &mut d.style;
    d.color = Color::GRAY;
    d.radii = RoundedRectRadii::from(3.0);
    s.margin = margin_all(px(5));
    s.padding = padding_all(px(5));
    s.flex_direction = FlexDirection::Column;
    s.justify_content = Some(JustifyContent::Center);
    s.align_items = Some(AlignItems::Center);
}

fn buttons(d: &mut Div) {
    let s = &mut d.style;
    s.size.width = Dimension::Length(50.0);
    s.justify_content = Some(JustifyContent::SpaceBetween);
}

fn inc_dec(d: &mut Div) {
    let s = &mut d.style;
    d.color = Color::rgb(0.1, 0.1, 0.1);
    d.radii = RoundedRectRadii::from(3.0);
    s.justify_content = Some(JustifyContent::Center);
    s.align_items = Some(AlignItems::Center);
    s.size = size_all(px(22));
}

fn text_dark(text: &mut Text) {
    text.color = Color::BLACK;
}

fn text_light(text: &mut Text) {
    text.color = Color::WHITE;
}