use gewy::kurbo::RoundedRectRadii;
use gewy::peniko::Color;
use gewy::*;
use gewy::taffy::*;

fn main() {
    env_logger::init();
    let fonts = FontDB::load("assets/fonts/Roboto-Regular.ttf").unwrap();
    App::new(fonts).start(|ctx| {
        let widget = multicounter::init(&mut ctx.store);
        ctx.add_window(Window::new(512, 512, widget));
    });
}

/// -------------- MULTICOUNTER ---------------------
mod multicounter {

    use gewy::*;
    use gewy::taffy::*;
    use crate::counter;
    use crate::counter::counter;
    use crate::*;

    pub struct State {
        counter_sum: i32,
        counter_handles: Vec<Handle<counter::State>>,
    }
    impl FromStore for State {
        fn from_store(store: &mut Store) -> Self {
            Self {
                counter_sum: 0,
                counter_handles: vec![ store.init() ],
            }
        }
    }

    #[derive(Clone)]
    enum Msg { Add, Remove }

    fn update(state_id: Id<State>, store: &mut Store, message: DynMessage) {
        let message = message.downcast_ref::<Msg>().unwrap();
        match message {
            Msg::Add => {
                let counter_handle = store.init();
                let state = store.get_mut(&state_id);
                state.counter_handles.push(counter_handle);
            }
            Msg::Remove => {
                let multicounter = store.get_mut(&state_id);
                multicounter.counter_handles.pop();
            }
        }
    }

    fn view(state: &State, _store: &Store, v: &mut View) {
        div_begin(root_cls, v);
            for counter_handle in &state.counter_handles {
                counter(counter_handle.clone(), v);
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

    pub fn init(store: &mut Store) -> impl Widget {
        let state = store.init::<State>();
        create_comp(state.clone(), update, view)
    }

    fn root_cls(d: &mut Div) {
        d.style.flex_direction = FlexDirection::Column;
        d.style.align_items = Some(AlignItems::Center);
    }
}


/// Defines the counter component.
mod counter {

    use gewy::*;
    use crate::*;
   
    #[derive(Default)]
    pub struct State(i32);

    #[derive(Clone)]
    enum Msg { Increment, Decrement }

    fn update(state_id: Id<State>, store: &mut Store, message: DynMessage) {
        let message: &Msg = message.downcast_ref().unwrap();
        let state = store.get_mut(&state_id);
        match message {
            Msg::Increment => state.0 += 1,
            Msg::Decrement => state.0 -= 1,
        }
    }

    fn view(state: &State, _store: &Store, v: &mut View) {
        let count_text = format!("Count: {}", state.0);
        col_begin(counter_box_cls, v);
            text(count_text, dark_text_cls, v);
            row_begin(small_box_cls, v);
                small_text_button("+", Msg::Increment, v);
                small_text_button("-", Msg::Decrement, v);
            end(v);
        end(v);
    }

    pub fn create(state: Handle<State>) -> impl Widget {
        create_comp(state.clone(), update, view)
    }

    pub fn counter(state: Handle<State>, v: &mut View) {
        let widget = create(state);
        v.insert(widget);
    }
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
