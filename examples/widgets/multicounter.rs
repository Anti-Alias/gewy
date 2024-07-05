use gewy::prelude::*;
use crate::widgets::counter::{counter, CounterState, CounterEvent};

type UParams<'a> = UpdateParams<'a, MulticounterState, MulticounterMsg>;
type VParams<'a, 'b> = ViewParams<'a, 'b, MulticounterState>;

pub fn multicounter(state: Id<MulticounterState>, v: &mut View) {
    let widget = Comp::new("multicounter", state, update, (), view);
    v.insert(widget);
}

pub struct MulticounterState {
    counter_sum: i32,
    counters: Vec<Id<CounterState>>,
}

impl FromStore for MulticounterState {
    fn from_store(store: &mut Store) -> Self {
        Self {
            counter_sum: 0,
            counters: vec![
                store.init::<CounterState>(),
            ],
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum MulticounterMsg { Add, Remove, Sync }

fn update(mut params: UParams) {
    match &params.msg {
        MulticounterMsg::Sync => compute_sum(&mut params),
        MulticounterMsg::Add => push_counter(&mut params),
        MulticounterMsg::Remove => {
            pop_counter(&mut params);
            compute_sum(&mut params);
        },
    }
}

fn view(mut params: VParams) {
    let (state, v) = params.state_view();
    let add_mapper = (ButtonEvent::Released, MulticounterMsg::Add);
    let rem_mapper = (ButtonEvent::Released, MulticounterMsg::Remove);
    let count_mapper = (CounterEvent::Changed, MulticounterMsg::Sync);
    let total = format!("Total: {}", state.counter_sum);
    col(cls::root).beg(v);
        text(total, cls::nop).ins(v);
        for counter_id in &state.counters {
            counter(counter_id.clone(), count_mapper, v);
        }
        row(cls::nop).beg(v);
            if state.counters.len() < 6 {
                text_button("Add Counter", add_mapper, v);
            }
            if state.counters.len() > 1 {
                text_button("Remove Counter", rem_mapper, v);
            }
        end(v);
    end(v);
}

fn push_counter(params: &mut UParams) {
    let counter = params.store.init::<CounterState>();
    params.state_mut().counters.push(counter);
}

fn pop_counter(params: &mut UParams) {
    params.state_mut().counters.pop();
}

fn compute_sum(params: &mut UParams) {
    let store = &params.store;
    let state = params.state();
    let counter_sum: i32 = state.counters.iter()
        .map(|state_id| store.get(state_id))
        .map(|state| state.0)
        .sum();
    params.state_mut().counter_sum = counter_sum;
}


fn text_button(txt: impl ToUiString, mapper: impl Mapper, v: &mut View) {
    button(cls::text_button).map(mapper).beg(v);
        text(txt, cls::light_text).ins(v);
    end(v);
}


mod cls {

    use gewy::*;
    use crate::*;

    pub fn nop<W>(_w: &mut W) {}

    pub fn root(d: &mut Div) {
        d.style.align_items = Some(AlignItems::Center);
    }

    pub fn text_button(b: &mut Button) {
        let s = &mut b.style;
        b.color = Color::rgb(0.1, 0.1, 0.1);
        b.radii = RoundedRectRadii::from(3.0);
        s.justify_content = Some(JustifyContent::Center);
        s.align_items = Some(AlignItems::Center);
        s.margin = margin_all(px(5.0));
        s.padding = padding_all(px(5.0));
    }

    pub fn light_text(text: &mut Text) {
        text.color = Color::WHITE;
    }
}