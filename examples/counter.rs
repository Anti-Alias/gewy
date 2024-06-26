use gewy::kurbo::RoundedRectRadii;
use gewy::peniko::Color;
use gewy::*;
use gewy::taffy::*;

fn main() {
    env_logger::init();
    let fonts = FontDB::load("assets/fonts/Roboto-Regular.ttf").unwrap();
    App::new(fonts).start(|ctx| {
        let app_handle: Handle<AppState> = ctx.init_state();
        let app_widget = Comp::new(app_handle, app_c, app_fn);
        ctx.add_window(Window::new(512, 512, app_widget));
        
    });
}

pub struct AppState {
    counters: Vec<Handle<CounterState>>,
}

impl State for AppState {
    fn bind(&self, bindings: &mut StateBindings) {
        for counter in &self.counters {
            bindings.add(counter);
        }
    }
}

impl FromStore for AppState {
    fn from_store(store: &mut Store) -> Self {
        Self {
            counters: vec![ store.init() ],
        }
    }
}

#[derive(Default)]
pub struct CounterState(i32);
impl State for CounterState {}


// --------------- Widget functions --------------- 
fn app_fn(id: Id<AppState>, store: &Store, v: &mut View) {
    let add_listener = move |evt: ButtonEvent, mut ctx: EventCtx| {
        if evt != ButtonEvent::Released { return };
        let counter_handle = ctx.init_state::<CounterState>();
        let state = ctx.state_mut(id);
        state.counters.push(counter_handle);
    };
    let rem_listener = move |evt: ButtonEvent, mut ctx: EventCtx| {
        if evt != ButtonEvent::Released { return };
        let state = ctx.state_mut(id);
        state.counters.pop();
    };
    let state = store.get(id);

    let counter_sum: i32 = state.counters.iter()
        .map(|handle| store.get(handle.id()))
        .map(|state| state.0)
        .sum();
    if counter_sum >= 10 {
        text("That's a lot of handles!", (), v);
    }

    for counter_handle in &state.counters {
        counter(counter_handle, v);
    }
    div_begin(nop_c, v);
        text_button("Add Counter", add_listener, v);
        text_button("Remove Counter", rem_listener, v);
    end(v);
}

fn counter(handle: &Handle<CounterState>, v: &mut View) {
    let counter_widget = Comp::new(handle.clone(), counter_c, counter_fn);
    v.insert(counter_widget);
}

fn counter_fn(id: Id<CounterState>, store: &Store, v: &mut View) {
    let state = store.get(id);
    let count_text = format!("Count: {}", state.0);
    let inc = move |evt: ButtonEvent, mut ctx: EventCtx| {
        if evt != ButtonEvent::Released { return };
        let state = ctx.state_mut(id);
        state.0 += 1;
    };
    let dec = move |evt: ButtonEvent, mut ctx: EventCtx| {
        if evt != ButtonEvent::Released { return };
        let state = ctx.state_mut(id);
        state.0 -= 1;
    };
    div_begin(c_counter_cont, v);
        text(count_text, text_dark_c, v);
        div_begin(inc_dec_c, v);
            small_text_button("+", inc, v);
            small_text_button("-", dec, v);
        end(v);
    end(v);
}

fn text_button(
    txt: impl ToGewyString,
    listener: impl Listener<ButtonEvent>,
    v: &mut View
) {
    button_begin(button_c, listener, v);
        text(txt, text_light_c, v);
    end(v);
}

fn small_text_button(
    txt: impl ToGewyString,
    listener: impl Listener<ButtonEvent>,
    v: &mut View
) {
    button_begin(small_button_c, listener, v);
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
