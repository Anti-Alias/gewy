use gewy::*;
use kurbo::RoundedRectRadii;
use peniko::Color;
use taffy::{AlignItems, Dimension, JustifyContent};

type UParams<'a> = UpdateParams<'a, CounterState, CounterMsg>;
type VParams<'a, 'b> = ViewParams<'a, 'b, CounterState>;


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

fn view(mut params: VParams) {
    let (state, v) = params.state_view();
    let txt = format!("Count: {}", state.0);
    let inc_map = (ButtonEvent::Released, CounterMsg::Increment);
    let dec_map = (ButtonEvent::Released, CounterMsg::Decrement);
    col_begin(counter_box_cls, v);
        text(txt, dark_text_cls, v);
        row_begin(small_box_cls, v);
            small_text_button("+", inc_map, v);
            small_text_button("-", dec_map, v);
        end(v);
    end(v);
}


fn small_text_button(txt: impl ToUiString, mapper: impl Mapper, v: &mut View) {
    button_begin(small_button_cls, mapper, v);
        text(txt, light_text_cls, v);
    end(v);
}

fn small_button_cls(b: &mut Button) {
    let s = &mut b.style;
    b.color = Color::rgb(0.1, 0.1, 0.1);
    b.radii = RoundedRectRadii::from(3.0);
    s.justify_content = Some(JustifyContent::Center);
    s.align_items = Some(AlignItems::Center);
    s.size = size_all(px(22));
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

fn dark_text_cls(text: &mut Text) {
    text.color = Color::BLACK;
}

fn light_text_cls(text: &mut Text) {
    text.color = Color::WHITE;
}

fn small_box_cls(d: &mut Div) {
    let s = &mut d.style;
    s.size.width = Dimension::Length(50.0);
    s.justify_content = Some(JustifyContent::SpaceBetween);
}