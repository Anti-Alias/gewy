use multicounter::multicounter;
use gewy::kurbo::RoundedRectRadii;
use gewy::peniko::Color;
use gewy::*;
use gewy::taffy::*;

fn main() {
    env_logger::init();
    let fonts = FontDB::load("assets/fonts/Roboto-Regular.ttf").unwrap();
    App::new(fonts)
        .on(AppEvent::Start, |ctx| ctx.create_window(512, 512, root))
        .start();
}

fn root(store: &mut Store, v: &mut View) {
    let state = multicounter::init(store);
    multicounter(state, v);
}

/// -------------- MULTICOUNTER ---------------------
mod multicounter {

    use gewy::*;
    use crate::counter;
    use crate::counter::counter;
    use crate::*;

    pub struct State {
        counter_sum: i32,
        counters: Vec<Id<counter::State>>,
    }

    #[derive(Clone)]
    enum Msg { Add, Remove }

    fn update(mut params: UpdateParams<State, Msg>) {
        match params.msg {
            Msg::Add => {
                let counter = counter::create_state(params.store);
                params.state_mut().counters.push(counter);
            }
            Msg::Remove => {
                params.state_mut().counters.pop();
            }
        }
    }

    fn view(mut params: ViewParams<State>) {
        let (state, v) = params.state_view();
        div_begin(root_cls, v);
            for counter_id in &state.counters {
                counter(counter_id.clone(), v);
            }
            div_begin(nop_cls, v);
                text_button("Add Counter", Msg::Add, v);
                text_button("Remove Counter", Msg::Remove, v);
            end(v);
            if state.counter_sum >= 10 {
                text("Total is at least 10!", nop_cls, v);
            }
        end(v);
    }

    pub fn init(store: &mut Store) -> Id<State> {
        let state = State {
            counter_sum: 0,
            counters: vec![
                counter::create_state(store),
            ],
        };
        store.create(state)
    }

    pub fn multicounter(state: Id<State>, v: &mut View) {
        let widget = Comp::root(state, update, view).with_name("app");
        v.insert(widget);
    }
}


/// Defines the counter component (SMUV = State, Message, Update, View).
mod counter {
    use gewy::*;
    use crate::*;

    /// S: Counter state type
    pub struct State(i32);

    /// M: Message type
    #[derive(Clone)]
    pub enum Msg { Increment, Decrement }

    /// U: Update function
    fn update(mut params: UpdateParams<State, Msg>) {
        match params.msg {
            Msg::Increment => params.state_mut().0 += 1,
            Msg::Decrement => params.state_mut().0 -= 1,
        }
    }

    /// V: View function
    fn view(mut params: ViewParams<State>) {
        let (state, v) = params.state_view();
        let count_text = format!("Count: {}", state.0);
        col_begin(counter_box_cls, v);
            text(count_text, dark_text_cls, v);
            row_begin(small_box_cls, v);
                small_text_button("+", Msg::Increment, v);
                small_text_button("-", Msg::Decrement, v);
            end(v);
        end(v);
    }

    // Insertion function
    pub fn counter(state: Id<State>, v: &mut View) {
        let widget = create_widget(state);
        v.insert(widget);
    }

    pub fn create_widget(state_id: Id<State>) -> impl Widget {
        Comp::new(state_id, update, view).with_name("counter")
    }

    pub fn create_state(store: &mut Store) -> Id<State> {
        store.create(State(0))
    }
}

// ---------------- Stateless widget functions ----------------

fn text_button(txt: impl ToUiString, msg: impl Message, v: &mut View) {
    let cls = move |button: &mut Button| button.on_release(msg);
    button_begin((text_button_cls, cls), v);
        text(txt, light_text_cls, v);
    end(v);
}

fn small_text_button(txt: impl ToUiString, msg: impl Message, v: &mut View) {
    let cls = move |button: &mut Button| button.on_release(msg);
    button_begin((small_button_cls, cls), v);
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
