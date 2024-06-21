use std::path::Path;
use gewy::geom::RoundedRectRadii;
use gewy::paint::{Blob, Color, Font};
use gewy::{begin, div, end, text, Div, FontDB, GewyApp, GewyAppEvent, GewyContext, GewyWindow, Text, TextAlign, Renderer, Widget};
use gewy::taffy::*;

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
        s.size.width = Dimension::Percent(1.0);             // Width is 100% of its parent (in this case, the window).
        s.size.height = Dimension::Percent(1.0);            // Height is 100% of its parent (in this case, the window).
        s.flex_direction = FlexDirection::Column;           // Main axis is top-to-bottom.
        s.justify_content = Some(JustifyContent::Center);   // Content on main axis is centered.
        s.align_items = Some(AlignItems::Center);           // Content on cross axis is centered.
    }

    /// A UIRenderer is used to build the DOM tree underneath this [`Widget`].
    /// The elements inserted (divs, texts, buttons etc) are also [`Widget`]s.
    /// Other UI frameworks refer to these as "components" (React, Angular etc).
    /// Some privimitive [`Widget`] functions like div() allow for external configuration via callback functions called "classes".
    /// Most higher level [`Widget`]s do not provide this functionality, however.
    fn render(&self, r: &mut Renderer) {
        div((c_round, c_red), r);                   // Inserts div. Configured with 2 classes (c_round, c_red).
        div(c_gray, r);                             // Inserts div. Configured with 1 class (c_gray).
        begin(r);                                   // Causes subsequent inserts to be children of the last widget inserted (in this case, it was a "div").
            div((c_round, c_green), r);             // Inserts div 
            text("This is some text!", c_text, r);  // Inserts text
            div((c_round, c_yellow), r);            // Inserts div
        end(r);                                     // Causes subsequent inserts to move back to the parent widget (in this case, it's the AppWidget).
        div((c_round, c_blue), r);                  // Inserts div.
    }
}


// --------------- Classes --------------- 

fn c_text(t: &mut Text) {
    t.color = Color::BLACK;
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
    s.size.width = Dimension::Percent(0.8);
    s.size.height = Dimension::Length(256.0);
    s.flex_direction = FlexDirection::Column;
    d.color = Color::GRAY;
}