use gewy::prelude::*;

pub fn counter(state: Id<CounterState>, mapper: impl Mapper, v: &mut View) {
    let widget = Comp::new("counter", state, update, mapper, view);
    v.insert(widget);
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CounterEvent { Changed }

#[derive(Default, Debug)]
pub struct CounterState(pub i32);

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum CounterMsg { Increment, Decrement }

fn update(mut params: UParams) {
    match params.msg {
        CounterMsg::Increment => params.state_mut().0 += 1,
        CounterMsg::Decrement => params.state_mut().0 -= 1,
    }
    params.emit(CounterEvent::Changed);
}

fn view(state: &CounterState, _store: &Store) -> View {
    let mut v = View::new();
    let txt = format!("Count: {}", state.0);
    Col::new().class(counter_box).begin(&mut v);
        Text::new(txt).class(dark_text).insert(&mut v);
        Row::new().class(small_box).begin(&mut v);
            TextButton::new("+").class(small_button).release(CounterMsg::Increment).insert(&mut v);
            TextButton::new("-").class(small_button).release(CounterMsg::Decrement).insert(&mut v);
        Row::end(&mut v);
    Col::end(&mut v);
    v
}

type UParams<'a> = UpdateParams<'a, CounterState, CounterMsg>;

pub fn small_button(b: &mut TextButton) {
    let b = &mut b.button;
    let s = &mut b.style;
    b.color = Color::rgb(0.1, 0.1, 0.1);
    b.radii = RoundedRectRadii::from(3.0);
    s.justify_content = Some(JustifyContent::Center);
    s.align_items = Some(AlignItems::Center);
    s.size = size_all(px(22));
}

pub fn counter_box(d: &mut Div) {
    let s = &mut d.style;
    d.color = Color::GRAY;
    d.radii = RoundedRectRadii::from(3.0);
    s.margin = margin_all(px(5));
    s.padding = padding_all(px(5));
    s.justify_content = Some(JustifyContent::Center);
    s.align_items = Some(AlignItems::Center);
}

pub fn dark_text(text: &mut Text) {
    text.color = Color::BLACK;
}

pub fn small_box(d: &mut Div) {
    let s = &mut d.style;
    s.size.width = Dimension::Length(50.0);
    s.justify_content = Some(JustifyContent::SpaceBetween);
}
