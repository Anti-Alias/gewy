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

fn view(mut params: VParams) {
    let (state, v) = params.state_view();
    let txt = format!("Count: {}", state.0);
    let inc_map = (ButtonEvent::Released, CounterMsg::Increment);
    let dec_map = (ButtonEvent::Released, CounterMsg::Decrement);
    col(cls::counter_box).beg(v);
        text(txt, cls::dark_text).ins(v);
        row(cls::small_box).beg(v);
            small_text_button("+", inc_map, v);
            small_text_button("-", dec_map, v);
        end(v);
    end(v);
}

fn small_text_button(txt: impl ToUiString, mapper: impl Mapper, v: &mut View) {
    button(cls::small_button).map(mapper).beg(v);
        text(txt, cls::light_text).ins(v);
    end(v);
}

type UParams<'a> = UpdateParams<'a, CounterState, CounterMsg>;
type VParams<'a, 'b> = ViewParams<'a, 'b, CounterState>;

mod cls {

    use gewy::prelude::*;

    pub fn small_button(b: &mut Button) {
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

    pub fn light_text(text: &mut Text) {
        text.color = Color::WHITE;
    }

    pub fn small_box(d: &mut Div) {
        let s = &mut d.style;
        s.size.width = Dimension::Length(50.0);
        s.justify_content = Some(JustifyContent::SpaceBetween);
    }
}

