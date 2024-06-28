use gewy::kurbo::RoundedRectRadii;
use gewy::peniko::Color;
use gewy::*;
use gewy::taffy::*;

fn main() {
    env_logger::init();
    let fonts = FontDB::load("assets/fonts/Roboto-Regular.ttf").unwrap();
    App::new(fonts).start(|ctx| {
        let handle: Handle<MultiCounter> = ctx.store.init();
        let widget = Comp::new(handle, multicounter_cls, multicounter_fn);
        ctx.add_window(Window::new(512, 512, widget));
    });
}

pub struct MultiCounter {
    counter_sum: i32,
    counters: Vec<Handle<Counter>>,
}

impl State for MultiCounter {
    fn update(id: &Id<Self>, store: &mut Store) {
        let slf = store.get(id);
        let counter_sum: i32 = slf.counters.iter()
            .map(|counter_id| store.get(counter_id))
            .map(|counter| counter.0)
            .sum();
        let slf = store.get_mut(id);
        slf.counter_sum = counter_sum;
    }
}

impl FromStore for MultiCounter {
    fn from_store(store: &mut Store) -> Self {
        Self {
            counter_sum: 0,
            counters: vec![ store.init() ],
        }
    }
}

#[derive(Default)]
pub struct Counter(i32);
impl State for Counter {}


// View function
fn multicounter_fn(id: Id<MultiCounter>, store: &Store, v: &mut View) {
    let add_listener = listener::for_event(ButtonEvent::Released, move |ctx| {
        let counter_handle = ctx.store.init::<Counter>();
        let state = ctx.store.get_mut(&id);
        state.counters.push(counter_handle);
    });
    let rem_listener = listener::for_event(ButtonEvent::Released, move |ctx| {
        let state = ctx.store.get_mut(&id);
        state.counters.pop();        
    });

    let state = store.get(&id);
    for counter_handle in &state.counters {
        counter(counter_handle, listener::update(id), v);
    }
    div_begin(nop_cls, v);
        text_button("Add Counter", add_listener, v);
        text_button("Remove Counter", rem_listener, v);
    end(v);
    if state.counter_sum >= 10 {
        text("Total is at least 10!", nop_cls, v);
    }
}

fn counter(
    handle: &Handle<Counter>,
    listener: impl Listener<i32> + Clone,
    v: &mut View
) {
    let view_fn = move |id: Id<Counter>, store: &Store, view: &mut View| {
        let listener = listener.clone();
        counter_fn(id, store, listener, view);
    };
    let counter_widget = Comp::new(handle.clone(), counter_cls, view_fn);
    v.insert(counter_widget);
}

fn counter_fn(
    id: Id<Counter>,
    store: &Store,
    listener: impl Listener<i32> + Clone,
    v: &mut View
) {
    let state = store.get(&id);
    let count_text = format!("Count: {}", state.0);

    let listener_b = listener.clone();
    let on_inc = listener::for_event(ButtonEvent::Released, move |ctx| {
        let state = ctx.store.get_mut(&id);
        state.0 += 1;
        listener_b.handle(state.0, ctx);
    });
    let on_dec = listener::for_event(ButtonEvent::Released, move |ctx| {
        let state = ctx.store.get_mut(&id);
        state.0 -= 1;
        listener.handle(state.0, ctx);
    });

    col_begin(counter_box_cls, v);
        text(count_text, dark_text_cls, v);
        row_begin(small_box_cls, v);
            small_text_button("+", on_inc, v);
            small_text_button("-", on_dec, v);
        end(v);
    end(v);
}

fn text_button(
    txt: impl ToGewyString,
    listener: impl Listener<ButtonEvent>,
    v: &mut View
) {
    button_begin(button_cls, listener, v);
        text(txt, light_text_cls, v);
    end(v);
}

fn small_text_button(
    txt: impl ToGewyString,
    listener: impl Listener<ButtonEvent>,
    v: &mut View
) {
    button_begin(small_button_cls, listener, v);
        text(txt, light_text_cls, v);
    end(v);
}


// --------------- Classes --------------- 

fn multicounter_cls(s: &mut Style) {
    s.size = size_all(pc(1.0));
    s.flex_direction = FlexDirection::Column;
    s.justify_content = Some(JustifyContent::Center);
    s.align_items = Some(AlignItems::Center);
}

fn counter_cls(s: &mut Style) {
    s.justify_content = Some(JustifyContent::Center);
    s.align_items = Some(AlignItems::Center);
}

fn counter_box_cls(d: &mut Div) {
    let s = &mut d.style;
    d.color = Color::GRAY;
    d.radii = RoundedRectRadii::from(3.0);
    s.margin = margin_all(px(5));
    s.padding = padding_all(px(5));
    s.justify_content = Some(JustifyContent::Center);
    s.align_items = Some(AlignItems::Center);
}

fn small_box_cls(d: &mut Div) {
    let s = &mut d.style;
    s.size.width = Dimension::Length(50.0);
    s.justify_content = Some(JustifyContent::SpaceBetween);
}

fn button_cls(b: &mut Button) {
    let s = &mut b.style;
    b.color = Color::rgb(0.1, 0.1, 0.1);
    b.radii = RoundedRectRadii::from(3.0);
    s.justify_content = Some(JustifyContent::Center);
    s.align_items = Some(AlignItems::Center);
    s.margin = margin_all(px(5.0));
    s.padding = padding_all(px(5.0));
}

fn small_button_cls(b: &mut Button) {
    let s = &mut b.style;
    b.color = Color::rgb(0.1, 0.1, 0.1);
    b.radii = RoundedRectRadii::from(3.0);
    s.justify_content = Some(JustifyContent::Center);
    s.align_items = Some(AlignItems::Center);
    s.size = size_all(px(22));
}

fn dark_text_cls(text: &mut Text) {
    text.color = Color::BLACK;
}

fn light_text_cls(text: &mut Text) {
    text.color = Color::WHITE;
}
