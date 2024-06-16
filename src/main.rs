use std::path::Path;
use gewy::geom::RoundedRectRadii;
use gewy::paint::{Blob, Color, Font};
use gewy::{begin, div, end, text, Div, FontDB, GewyApp, GewyAppEvent, GewyContext, GewyWindow, Text, TextAlign, UIRenderer, Widget};
use gewy::layout::*;

fn main() {
    env_logger::init();
    let default_font = load_font("assets/fonts/arial.ttf").unwrap();
    let font_db = FontDB::new(default_font);
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

fn load_font(path: impl AsRef<Path>) -> Result<Font, std::io::Error> {
    let bytes = std::fs::read(path)?;
    let bytes = Blob::from(bytes);
    Ok(Font::new(bytes, 0))
}


struct AppWidget;
impl Widget for AppWidget {

    /// [`Widget`]s must provide a [`Style`], which is comparable to styles in css.
    /// Unlike css, [`Style`]s in [`gewy`] contain only layout information.
    fn style(&self, s: &mut Style) {
        s.size.width = Dimension::Percent(1.0);
        s.size.height = Dimension::Percent(1.0);
        s.flex_direction = FlexDirection::Column;
        s.justify_content = Some(JustifyContent::Center);
        s.align_items = Some(AlignItems::Center);
    }

    /// A UIRenderer is used to build the DOM tree underneath this [`Widget`].
    /// The elements inserted (divs, texts, buttons etc) are also [`Widget`].
    /// Other UI frameworks refer to these as "components" (React, Angular etc).
    /// Some privimitive [`Widget`]s like divs allow for their [`Style`]s to be externally configured via class callback functions (or tuples of these).
    /// Most higher level [`Widget`]s do not provide this functionality, however.
    fn render(&self, r: &mut UIRenderer) {
        div((c_round, c_red), r);           // Inserts div with no children. Configured with 2 classes (c_round, c_red).
        div(c_gray, r);                     // Inserts div. Configured with 1 class (c_gray).
        begin(r);                           // Causes subsequent inserts to be children of the last widget inserted (in this case, it was a "div").
            div((c_round, c_green), r);     // Inserts div with no children. 
            text("This", c_text, r);        // Inserts text. Configured with 2 classes (c_round, c_yellow).
            div((c_round, c_yellow), r);    // Insrts div with no children
        end(r);                             // Causes subsequent inserts to move back to the parent widget (in this case, it's the AppWidget).
        div((c_round, c_blue), r);          // Inserts div with no children. Configured with 2 classes (c_round, c_blue).
    }
}


// --------------- Classes --------------- 

fn c_text(t: &mut Text) {
    t.width = Some(Dimension::Length(128.0));
    t.height = Some(Dimension::Length(64.0));
    t.text_align = TextAlign::Center;
}

fn c_round(d: &mut Div) {
    d.radii = RoundedRectRadii::from_single_radius(5.0);
    d.style.size = Size::from_lengths(32.0, 32.0);
}

fn c_red(d: &mut Div) {
    d.color = Color::RED;
}

fn c_yellow(d: &mut Div) {
    let s = &mut d.style;
    s.size.width = Dimension::Length(64.0);
    s.size.height = Dimension::Length(64.0);
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
    s.size.width = Dimension::Percent(0.5);
    s.size.height = Dimension::Percent(129.0);
    s.flex_direction = FlexDirection::Column;
    d.color = Color::GRAY;
}