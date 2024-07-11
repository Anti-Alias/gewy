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

fn view(store: &mut Store) -> ViewCmds {
    let state = store.init::<CounterState>();
    let mut view = ViewCmds::new();
    let v = &mut view;
    col(cls::center).begin(v);
        div(cls::title_box).begin(v);
            text("Counter Example!", cls::nop).insert(v);
        end(v);
        counter(state, (), v);
    end(v);
    view
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