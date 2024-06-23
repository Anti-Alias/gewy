use gewy::kurbo::RoundedRectRadii;
use gewy::peniko::Color;
use gewy::{begin, button, button_begin, button_mut, div, div_begin, end, margin_all, nop_c, padding_all, pc, px, size_all, text, App, Button, Comp, Div, FontDB, FromStore, Id, Store, Text, ToGewyString, View, WidgetId, Window};
use gewy::taffy::*;

fn main() {
    env_logger::init();
    let font_db = FontDB::load("assets/fonts/Roboto-Regular.ttf").unwrap();

    App::new(font_db).start(|ctx| {
        let app_state = ctx.init_state::<AppState>();
        let app_widget_1 = Comp::new(app_state.clone(), app_c, app_fn);
        let app_widget_2 = Comp::new(app_state, app_c, app_fn);
        ctx.add_window(Window::new(512, 512, app_widget_1));
        ctx.add_window(Window::new(512, 512, app_widget_2));
    });
}

pub struct AppState {
    counters: Vec<Id<i32>>,
}

impl FromStore for AppState {
    fn from_store(store: &mut Store) -> Self {
        Self {
            counters: vec![ store.create(0) ],
        }
    }
}

// --------------- Widget functions --------------- 

fn app_fn(state: Id<AppState>, value: &AppState, v: &mut View) {
    let add_count = &mut WidgetId::default();
    let rem_count = &mut WidgetId::default();

    for counter_state in &value.counters {
        counter(&counter_state, v);
    }
    div_begin(nop_c, v);
        button_begin(button_c, add_count, v);
            text("Add Counter", text_light_c, v);
        end(v);
        button_begin(button_c, rem_count, v);
            text("Remove Counter", text_light_c, v);
        end(v);
    end(v);


    let state_a = state.clone();
    button_mut(*add_count, v).release(move |ctx| {
        let counter_state = ctx.init_state::<i32>();
        let value = ctx.state_mut(&state_a);
        value.counters.push(counter_state);
    });
    button_mut(*rem_count, v).release(move |ctx| {
        let value = ctx.state_mut(&state);
        value.counters.pop();
    });
}

fn counter(state: &Id<i32>, v: &mut View) {
    let counter_widget = Comp::new(state.clone(), counter_c, counter_fn);
    v.insert(counter_widget);
}

fn counter_fn(state: Id<i32>, value: &i32, v: &mut View) {
    let value = *value;
    let count_text = format!("Count: {value}");
    let inc = &mut WidgetId::default();
    let dec = &mut WidgetId::default();

    div_begin(c_counter_cont, v);
        text(count_text, text_dark_c, v);
        div_begin(inc_dec_c, v);
            button_dark("+", inc, v);
            button_dark("-", dec, v);
        end(v);
    end(v);

    let state_a = state.clone();
    button_mut(*inc, v).release(move |ctx| *ctx.state_mut(&state) += 1);
    button_mut(*dec, v).release(move |ctx| *ctx.state_mut(&state_a) -= 1);
}

fn button_dark(txt: impl ToGewyString, button_id: &mut WidgetId, v: &mut View) {
    button_begin(small_button_c, button_id, v);
        text(txt, text_light_c, v);
    end(v);
}


// --------------- Classes --------------- 

fn app_c(s: &mut Style) {
    s.size = size_all(pc(1.0));
    s.flex_direction = FlexDirection::Column;
    s.justify_content = Some(JustifyContent::Center);
    s.align_items = Some(AlignItems::Center);
}

fn counter_c(s: &mut Style) {
    s.justify_content = Some(JustifyContent::Center);
    s.align_items = Some(AlignItems::Center);
}

fn c_counter_cont(d: &mut Div) {
    let s = &mut d.style;
    d.color = Color::GRAY;
    d.radii = RoundedRectRadii::from(3.0);
    s.margin = margin_all(px(5));
    s.padding = padding_all(px(5));
    s.flex_direction = FlexDirection::Column;
    s.justify_content = Some(JustifyContent::Center);
    s.align_items = Some(AlignItems::Center);
}

fn inc_dec_c(d: &mut Div) {
    let s = &mut d.style;
    s.size.width = Dimension::Length(50.0);
    s.justify_content = Some(JustifyContent::SpaceBetween);
}

fn button_c(b: &mut Button) {
    let s = &mut b.style;
    b.color = Color::rgb(0.1, 0.1, 0.1);
    b.radii = RoundedRectRadii::from(3.0);
    s.justify_content = Some(JustifyContent::Center);
    s.align_items = Some(AlignItems::Center);
    s.margin = margin_all(px(5.0));
    s.padding = padding_all(px(5.0));
}

fn small_button_c(b: &mut Button) {
    let s = &mut b.style;
    b.color = Color::rgb(0.1, 0.1, 0.1);
    b.radii = RoundedRectRadii::from(3.0);
    s.justify_content = Some(JustifyContent::Center);
    s.align_items = Some(AlignItems::Center);
    s.size = size_all(px(22));
}

fn text_dark_c(text: &mut Text) {
    text.color = Color::BLACK;
}

fn text_light_c(text: &mut Text) {
    text.color = Color::WHITE;
}
