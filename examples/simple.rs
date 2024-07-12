use gewy::prelude::*;

fn main() {
    env_logger::init();
    let fonts = FontDB::load("assets/fonts/Roboto-Regular.ttf").unwrap();
    App::new(fonts)
        .on(AppEvent::Start, |ctx| ctx.create_window(512, 512, ui))
        .start();
}

fn ui(v: &mut View) {
    Col::new().class(root).begin(v);
        Div::new().class(red).insert(v);
        Col::new().class(gray).begin(v);
            Div::new().class(green).insert(v);
            Text::new("This is some text!").class(text).insert(v);
            Div::new().class(yellow).insert(v);
        Col::end(v);
        Div::new().class(blue).insert(v);
    Col::end(v);
}



// ---------- Class functions ----------
fn root(div: &mut Div) {
    let s = &mut div.style;
    s.size.width = pc(1.0);
    s.size.height = pc(1.0);
    s.justify_content = Some(JustifyContent::Center);
    s.align_items = Some(AlignItems::Center);
}

fn text(t: &mut Text) {
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
    s.size = size(pc(0.8), px(256));
}
