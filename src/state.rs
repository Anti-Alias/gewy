pub use derive::*;
use std::any::Any;

pub type StatePath<'a> = &'a [&'a str];

pub trait State: Any {
    fn state<'a>(&'a self, _name: &str) -> Option<&'a dyn State> {
        None
    }
    fn state_mut<'a>(&'a mut self, _name: &str) -> Option<&'a mut dyn State> {
        None
    }
    fn as_any_ref(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

pub(crate) fn query_state<'s, T: State>(
    mut state: &'s dyn State,
    mut path: StatePath,
) -> Option<&'s T> {
    loop {
        let Some((id, remainder)) = path.split_first() else {
            break;
        };
        let Some(sub_state) = state.state(id) else {
            return None;
        };
        state = sub_state;
        path = remainder;
    }
    let state = state.as_any_ref().downcast_ref::<T>()?;
    Some(state)
}

pub(crate) fn query_state_mut<'s, T: State>(
    mut state: &'s mut dyn State,
    mut path: StatePath,
) -> Option<&'s T> {
    loop {
        let Some((id, remainder)) = path.split_first() else {
            break;
        };
        let Some(sub_state) = state.state_mut(id) else {
            return None;
        };
        state = sub_state;
        path = remainder;
    }
    let state = state.as_any_mut().downcast_mut::<T>()?;
    Some(state)
}

#[cfg(test)]
mod test {

    use super::*;

    #[derive(Eq, PartialEq, Debug)]
    struct Parent {
        value: u32,
        child1: Child,
        child2: Child,
    }

    impl State for Parent {
        fn state<'a>(&'a self, name: &str) -> Option<&'a dyn State> {
            match name {
                "child1" => Some(&self.child1),
                "child2" => Some(&self.child2),
                _ => None,
            }
        }

        fn state_mut<'a>(&'a mut self, name: &str) -> Option<&'a mut dyn State> {
            match name {
                "child1" => Some(&mut self.child1),
                "child2" => Some(&mut self.child2),
                _ => None,
            }
        }

        fn as_any_ref(&self) -> &dyn Any {
            self
        }
        fn as_any_mut(&mut self) -> &mut dyn Any {
            self
        }
    }

    #[derive(Eq, PartialEq, Debug)]
    struct Child {
        value: i32,
        leaf: Leaf,
    }

    impl State for Child {
        fn state<'a>(&'a self, name: &str) -> Option<&'a dyn State> {
            match name {
                "leaf" => Some(&self.leaf),
                _ => None,
            }
        }

        fn state_mut<'a>(&'a mut self, name: &str) -> Option<&'a mut dyn State> {
            match name {
                "leaf" => Some(&mut self.leaf),
                _ => None,
            }
        }
        fn as_any_ref(&self) -> &dyn Any {
            self
        }
        fn as_any_mut(&mut self) -> &mut dyn Any {
            self
        }
    }

    #[derive(Eq, PartialEq, Debug)]
    struct Leaf {
        value: i32,
    }
    impl State for Leaf {
        fn as_any_ref(&self) -> &dyn Any {
            self
        }
        fn as_any_mut(&mut self) -> &mut dyn Any {
            self
        }
    }

    #[test]
    fn test_query_state() {
        let parent = Parent {
            value: 1,
            child1: Child {
                value: 2,
                leaf: Leaf { value: 3 },
            },
            child2: Child {
                value: 4,
                leaf: Leaf { value: 5 },
            },
        };
        let par = query_state::<Parent>(&parent, &[]);
        let ch1 = query_state::<Child>(&parent, &["child1"]);
        let ch2 = query_state::<Child>(&parent, &["child2"]);
        let lf1 = query_state::<Leaf>(&parent, &["child1", "leaf"]);
        let lf2 = query_state::<Leaf>(&parent, &["child2", "leaf"]);
        let none_a = query_state::<Leaf>(&parent, &["none", "leaf"]);
        let none_b = query_state::<Leaf>(&parent, &["child1", "none"]);
        let none_wrong_type = query_state::<Leaf>(&parent, &["child1"]);
        assert_eq!(par, Some(&parent));
        assert_eq!(ch1, Some(&parent.child1));
        assert_eq!(ch2, Some(&parent.child2));
        assert_eq!(lf1, Some(&parent.child1.leaf));
        assert_eq!(lf2, Some(&parent.child2.leaf));
        assert_eq!(none_a, None);
        assert_eq!(none_b, None);
        assert_eq!(none_wrong_type, None);
    }
}
