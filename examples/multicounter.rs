use gewy::kurbo::RoundedRectRadii;
use gewy::peniko::Color;
use gewy::*;
use gewy::taffy::*;

fn main() {
    env_logger::init();
    let fonts = FontDB::load("assets/fonts/Roboto-Regular.ttf").unwrap();
    App::new(fonts).start(|ctx| {
        let multicounter_state: Handle<MultiCounter> = ctx.store.init();
        let multicounter_widget = make_comp(multicounter_state, multicounter_reducer, multicounter_view);
        ctx.add_window(Window::new(512, 512, multicounter_widget));
    });
}

/// -------------- MULTICOUNTER ---------------------
pub struct MultiCounter {
    counter_sum: i32,
    counters: Vec<Handle<Counter>>,
}
impl FromStore for MultiCounter {
    fn from_store(store: &mut Store) -> Self {
        Self {
            counter_sum: 0,
            counters: vec![ store.init() ],
        }
    }
}

#[derive(Clone)]
enum MultiMsg { AddCounter, RemoveCounter }

fn multicounter_reducer(id: Id<MultiCounter>, store: &mut Store, message: DynMessage) {
    let message = message.downcast_ref::<MultiMsg>().unwrap();
    match message {
        MultiMsg::AddCounter => {
            let new_counter: Handle<Counter> = store.init();
            let multicounter = store.get_mut(&id);
            multicounter.counters.push(new_counter);
        }
        MultiMsg::RemoveCounter => {
            let multicounter = store.get_mut(&id);
            multicounter.counters.pop();
        }
    }
}

fn multicounter_view(multi: Id<MultiCounter>, store: &Store, v: &mut View) {
    let multi = store.get(&multi);
    div_begin(root_cls, v);
        for counter_handle in &multi.counters {
            counter(counter_handle, v);
        }
        div_begin(nop_cls, v);
            text_button("Add Counter", MultiMsg::AddCounter, v);
            text_button("Remove Counter", MultiMsg::RemoveCounter, v);
        end(v);
        if multi.counter_sum >= 10 {
            text("Total is at least 10!", nop_cls, v);
        }
    end(v);
}

fn root_cls(d: &mut Div) {
    d.style.flex_direction = FlexDirection::Column;
    d.style.align_items = Some(AlignItems::Center);
}


/// -------------- COUNTER ---------------------
#[derive(Default)]
struct Counter(i32);

#[derive(Clone)]
enum CounterMsg { Increment, Decrement }

fn counter_reducer(id: Id<Counter>, store: &mut Store, message: DynMessage) {
    let message: &CounterMsg = message.downcast_ref().unwrap();
    match message {
        CounterMsg::Increment => store.get_mut(&id).0 += 1,
        CounterMsg::Decrement => store.get_mut(&id).0 -= 1,
    }
}


fn counter(
    handle: &Handle<Counter>,
    v: &mut View
) {
    let counter_widget = make_comp(handle.clone(), counter_reducer, counter_view);
    v.insert(counter_widget);
}

fn counter_view(
    id: Id<Counter>,
    store: &Store,
    v: &mut View
) {
    let state = store.get(&id);
    let count_text = format!("Count: {}", state.0);

    col_begin(counter_box_cls, v);
        text(count_text, dark_text_cls, v);
        row_begin(small_box_cls, v);
            small_text_button("+", CounterMsg::Increment, v);
            small_text_button("-", CounterMsg::Decrement, v);
        end(v);
    end(v);
}

fn text_button(
    txt: impl ToGewyString,
    message: impl Message,
    v: &mut View
) {
    let cls = move |button: &mut Button| button.on_release(message);
    button_begin((text_button_cls, cls), v);
        text(txt, light_text_cls, v);
    end(v);
}

fn small_text_button(
    txt: impl ToGewyString,
    message: impl Message,
    v: &mut View
) {
    let cls = move |button: &mut Button| button.on_release(message);
    button_begin((small_button_cls, cls), v);
        text(txt, light_text_cls, v);
    end(v);
}


// --------------- Classes --------------- 

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

fn text_button_cls(b: &mut Button) {
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
