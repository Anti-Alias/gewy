use gewy::prelude::*;

pub struct Counter { state: Id<CounterState> }

impl Counter {
    pub fn new(state: Id<CounterState>) -> Self {
        Self { state }
    }
    pub fn insert(self, view: &mut View) {
        view.insert_component(self);
    }
}

impl Component for Counter {

    type State = CounterState;
    type Message = CounterMsg;

    fn update(&self, params: UParams<Self>) {
        match params.msg {
            CounterMsg::Increment  => params.state.0 += 1,
            CounterMsg::Decrement  => params.state.0 -= 1,
            CounterMsg::Reset      => params.state.0 = 0,
        }
    }

    fn view(&self, params: VParams<Self>) {
        let (v, state) = (params.view, params.state);
        let text = format!("Count: {}", state.0);
        Col::new().class(counter).begin(v);
            Text::new(text).class(dark).insert(v);
            Row::new().begin(v);
                TextButton::new("+").class(small).release(CounterMsg::Increment).insert(v);
                TextButton::new("-").class(small).release(CounterMsg::Decrement).insert(v);
                TextButton::new("Reset").class(button).release(CounterMsg::Reset).insert(v);
            Row::end(v);
        Col::end(v);
    }

    fn state(&self) -> &Id<CounterState> { &self.state }
}

#[derive(Default, Debug)]
pub struct CounterState(pub i32);

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum CounterMsg { Increment, Decrement, Reset }
impl MessageType for CounterMsg {}

pub fn button(b: &mut TextButton) {
    let b = &mut b.button;
    b.color = Color::rgb(0.1, 0.1, 0.1);
    b.radii = RoundedRectRadii::from(3.0);
    b.style.padding = padding(px(2), px(2), px(0), px(5));
    b.style.margin = margin(px(0), px(2), px(0), px(2));
}

pub fn small(b: &mut TextButton) {
    let b = &mut b.button;
    let s = &mut b.style;
    b.color = Color::rgb(0.1, 0.1, 0.1);
    b.radii = RoundedRectRadii::from(3.0);
    s.justify_content = Some(JustifyContent::Center);
    s.align_items = Some(AlignItems::Center);
    s.size = size_all(px(22));
    b.style.margin = margin(px(0), px(2), px(0), px(2));
}

pub fn counter(d: &mut Div) {
    let s = &mut d.style;
    d.color = Color::GRAY;
    d.radii = RoundedRectRadii::from(3.0);
    s.margin = margin_all(px(5));
    s.padding = padding_all(px(5));
    s.justify_content = Some(JustifyContent::Center);
    s.align_items = Some(AlignItems::Center);
}

pub fn dark(text: &mut Text) {
    text.color = Color::BLACK;
}
