mod widgets;

use gewy::prelude::*;
use widgets::counter::*;


fn main() {
    env_logger::init();
    let fonts = FontDB::load("assets/fonts/Roboto-Regular.ttf").unwrap();
    App::new(fonts)
        .on(AppEvent::Start, start)
        .start();
}

fn start(ctx: &mut AppCtx) {
    ctx.create_window(512, 512, view);
}

fn view(store: &mut Store) -> View {
    let state = store.init::<CounterState>();
    let mut v = View::new();
    Col::new().class(center).begin(&mut v);
        Div::new().class(title_box).begin(&mut v);
            Text::new("Counter Example!").insert(&mut v);
        Div::end(&mut v);
        counter(state, (), &mut v);
    Col::end(&mut v);
    v
}

fn center(d: &mut Div) {
    d.style.align_items = Some(AlignItems::Center);
}

fn title_box(d: &mut Div) {
    d.style.margin = margin(px(0), px(0), px(10), px(0));
}