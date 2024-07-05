use multicounter::multicounter;
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
    ctx.create_window(512, 512, root);
}

fn root(store: &mut Store, v: &mut View) {
    let state = store.init::<multicounter::State>();
    col_begin((), v);
        multicounter(state, (), v);
    end(v);
}

/// -------------- MULTICOUNTER ---------------------
mod multicounter {

    use gewy::*;
    use crate::counter::{self, CounterEvent};
    use crate::counter::counter;
    use crate::*;

    type UParams<'a> = UpdateParams<'a, State, Msg>;

    pub struct State {
        counter_sum: i32,
        counters: Vec<Id<counter::State>>,
    }

    impl FromStore for State {
        fn from_store(store: &mut Store) -> Self {
            Self {
                counter_sum: 0,
                counters: vec![
                    store.init::<counter::State>(),
                ],
            }
        }
    }

    #[derive(Copy, Clone, Eq, PartialEq, Debug)]
    enum Msg { Add, Remove, Sync }


    fn update(mut params: UParams) {
        match &params.msg {
            Msg::Sync => sync(&mut params),
            Msg::Add => add(&mut params),
            Msg::Remove => {
                remove(&mut params);
                sync(&mut params);
            },
        }
    }

    fn add(params: &mut UpdateParams<State, Msg>) {
        let counter = counter::create_state(params.store);
        params.state_mut().counters.push(counter);
    }

    fn remove(params: &mut UpdateParams<State, Msg>) {
        params.state_mut().counters.pop();
    }

    fn sync(params: &mut UpdateParams<State, Msg>) {
        let store = &params.store;
        let state = params.state();
        let counter_sum: i32 = state.counters.iter()
            .map(|state_id| store.get(state_id))
            .map(|state| state.0)
            .sum();
        params.state_mut().counter_sum = counter_sum;
    }

    fn view(mut params: ViewParams<State>) {
        let (state, v) = params.state_view();
        let add_mapper = (ButtonEvent::Released, Msg::Add);
        let rem_mapper = (ButtonEvent::Released, Msg::Remove);
        let count_mapper = (CounterEvent::Changed, Msg::Sync);
        let sum_str = format!("Total: {}", state.counter_sum);
        div_begin(root_cls, v);
            text(sum_str, (), v);
            for counter_id in &state.counters {
                counter(counter_id.clone(), count_mapper, v);
            }
            div_begin(nop_cls, v);
                if state.counters.len() < 6 {
                    text_button("Add Counter", add_mapper, v);
                }
                if state.counters.len() > 1 {
                    text_button("Remove Counter", rem_mapper, v);
                }
            end(v);
        end(v);
    }

    pub fn multicounter(state: Id<State>, mapper: impl Mapper, v: &mut View) {
        let widget = Comp::new(state, mapper, update, view).with_name("app");
        v.insert(widget);
    }
}


/// Defines the counter component (SMUV = State, Message, Update, View).
mod counter {
    use gewy::*;
    use crate::*;

    /// S: Counter state type
    #[derive(Default)]
    pub struct State(pub i32);

    /// M: Message type
    #[derive(Copy, Clone, Eq, PartialEq, Debug)]
    enum Msg { Increment, Decrement }

    #[derive(Copy, Clone, Eq, PartialEq)]
    pub enum CounterEvent { Changed }

    /// U: Update function
    fn update(mut params: UpdateParams<State, Msg>) {
        match params.msg {
            Msg::Increment => params.state_mut().0 += 1,
            Msg::Decrement => params.state_mut().0 -= 1,
        }
        params.emit(CounterEvent::Changed);
    }

    /// V: View function
    fn view(mut params: ViewParams<State>) {
        let (state, v) = params.state_view();
        let count_text = format!("Count: {}", state.0);
        let inc_mapper = (ButtonEvent::Released, Msg::Increment);
        let dec_mapper = (ButtonEvent::Released, Msg::Decrement);
        col_begin(counter_box_cls, v);
            text(count_text, dark_text_cls, v);
            row_begin(small_box_cls, v);
                small_text_button("+", inc_mapper, v);
                small_text_button("-", dec_mapper, v);
            end(v);
        end(v);
    }

    // Insertion function
    pub fn counter(state: Id<State>, mapper: impl Mapper, v: &mut View) {
        let widget = Comp::new(state, mapper, update, view).with_name("counter");
        v.insert(widget);
    }

    pub fn create_state(store: &mut Store) -> Id<State> {
        store.create(State(0))
    }
}

// ---------------- Stateless widget functions ----------------

fn text_button(txt: impl ToUiString, mapper: impl Mapper, v: &mut View) {
    button_begin(text_button_cls, mapper, v);
        text(txt, light_text_cls, v);
    end(v);
}

fn small_text_button(txt: impl ToUiString, mapper: impl Mapper, v: &mut View) {
    button_begin(small_button_cls, mapper, v);
        text(txt, light_text_cls, v);
    end(v);
}


// --------------- Classes --------------- 
fn root_cls(d: &mut Div) {
    d.style.flex_direction = FlexDirection::Column;
    d.style.align_items = Some(AlignItems::Center);
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
