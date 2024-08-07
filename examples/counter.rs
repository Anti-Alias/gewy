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
    let state = ctx.store.init::<CounterState>();
    ctx.create_window(512, 512, move |v| ui(state, v));
}

fn ui(state: Id<CounterState>, v: &mut View) {
    Col::new().class(center).begin(v);
        Div::new().class(title_box).begin(v);
            Text::new("Counter Example!").insert(v);
        Div::end(v);
        Counter::new(state).insert(v);
    Col::end(v);
}

fn center(d: &mut Div) {
    d.style.align_items = Some(AlignItems::Center);
}

fn title_box(d: &mut Div) {
    d.style.margin = margin(px(0), px(0), px(10), px(0));
}