mod widgets;
use widgets::*;

use multicounter::*;
use gewy::kurbo::RoundedRectRadii;
use gewy::peniko::Color;
use gewy::*;
use gewy::taffy::*;


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
    let state = store.init::<MulticounterState>();
    col_begin((), v);
        div_begin(cls::title_box, v);
            text("Multicounter Example!", (), v);
        end(v);
        multicounter(state, v);
    end(v);
}



mod cls {
    use gewy::*;
    pub fn title_box(d: &mut Div) {
        d.style.margin = margin(px(0), px(0), px(10), px(0));
    }
}