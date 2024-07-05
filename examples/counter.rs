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

fn view(store: &mut Store, v: &mut View) {
    let state = store.init::<CounterState>();
    col(cls::center).beg(v);
        div(cls::title_box).beg(v);
            text("Counter Example!", cls::nop).ins(v);
        end(v);
        counter(state, (), v);
    end(v);
}



mod cls {
    use gewy::prelude::*;

    pub fn nop<W>(_w: &mut W) {}

    pub fn center(d: &mut Div) {
        d.style.align_items = Some(AlignItems::Center);
    }

    pub fn title_box(d: &mut Div) {
        d.style.margin = margin(px(0), px(0), px(10), px(0));
    }
}