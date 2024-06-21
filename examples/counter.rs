use std::path::Path;
use gewy::kurbo::RoundedRectRadii;
use gewy::peniko::{Blob, Color, Font};
use gewy::{div_begin, end, margin, padding, size, text, Div, FontDB, GewyApp, GewyAppEvent, GewyContext, GewyWindow, Renderer, Text, ToGewyString, Widget};
use gewy::taffy::*;

fn main() {
    env_logger::init();
    let default_font = load_font("assets/fonts/Roboto-Regular.ttf").unwrap();
    let font_db = FontDB::new(default_font);
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
    ctx.add_window(GewyWindow::new(512, 512, AppWidget));
    ctx.add_window(GewyWindow::new(512, 512, AppWidget));
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
    s.margin = margin::px_all(5.0);
    s.padding = padding::px_all(5.0);
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
    s.align_items = AlignItems::Center.into();
    s.size = size::px(22.0, 22.0);
}

fn text_dark(text: &mut Text) {
    text.color = Color::BLACK;
}

fn text_light(text: &mut Text) {
    text.color = Color::WHITE;
}