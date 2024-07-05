mod widgets;

use gewy::prelude::*;
use widgets::multicounter::*;


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
    col(cls::center, v).begin();
        div(cls::title_box, v).begin();
            text("Multicounter Example!", (), v);
        end(v);
        multicounter(state, v);
    end(v);
}



mod cls {
    use gewy::*;
    use taffy::AlignItems;

    pub fn center(d: &mut Div) {
        d.style.align_items = Some(AlignItems::Center);
    }

    pub fn title_box(d: &mut Div) {
        d.style.margin = margin(px(0), px(0), px(10), px(0));
    }
}