use gewy::prelude::*;

fn main() {
    env_logger::init();
    let fonts = FontDB::load("assets/fonts/Roboto-Regular.ttf").unwrap();
    App::new(fonts)
        .on(AppEvent::Start, |ctx| ctx.create_window(512, 512, root))
        .start();
}

// View function
fn root(_store: &mut Store, v: &mut View) {
    col(cls::root).beg(v);
        div(cls::red).ins(v);
        col(cls::gray).beg(v);
            div(cls::green).ins(v);
            text("This is some text!", cls::text).ins(v);
            div(cls::yellow).ins(v);
        end(v);
        div(cls::blue).ins(v);
    end(v);
}


// ---------- Class functions ----------
mod cls {

    use gewy::prelude::*;

    pub fn root(div: &mut Div) {
        let s = &mut div.style;
        s.size.width = pc(1.0);
        s.size.height = pc(1.0);
        s.justify_content = Some(JustifyContent::Center);
        s.align_items = Some(AlignItems::Center);
    }

    pub fn text(t: &mut Text) {
        t.color = Color::BLACK;
        t.text_align = TextAlign::Center;
    }

    pub fn red(d: &mut Div) {
        d.radii = RoundedRectRadii::from_single_radius(5.0);
        d.style.size = size_all(px(32));
        d.color = Color::RED;
    }

    pub fn yellow(d: &mut Div) {
        let s = &mut d.style;
        d.radii = RoundedRectRadii::from_single_radius(5.0);
        d.color = Color::YELLOW;
        s.size = size_all(px(64));
        s.margin = margin_all(px(5));
        s.size = size_all(px(32));
    }

    pub fn green(d: &mut Div) {
        d.radii = RoundedRectRadii::from_single_radius(5.0);
        d.style.size = size_all(px(32));
        d.color = Color::GREEN;
        d.style.margin = margin_all(px(5));
    }

    pub fn blue(d: &mut Div) {
        d.radii = RoundedRectRadii::from_single_radius(5.0);
        d.style.size = size_all(px(32));
        d.color = Color::BLUE;
    }

    pub fn gray(d: &mut Div) {
        let s = &mut d.style;
        d.radii = RoundedRectRadii::from_single_radius(20.0);
        d.color = Color::GRAY;
        s.flex_direction = FlexDirection::Column;
        s.justify_content = Some(JustifyContent::Center);
        s.align_items = Some(AlignItems::Center);
        s.size = size(pc(0.8), px(256));
    }
}