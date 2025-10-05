pub use derive::*;
use std::any::Any;

/// Storage of a single value, and child states.
pub trait State {

    fn value(&self) -> &dyn Any;

    fn child<'a>(&'a self, _name: &str) -> Option<&'a dyn Any> {
        None
    }

    fn child_mut<'a>(&'a mut self, _name: &str) -> Option<&'a mut dyn Any> {
        None
    }
}


#[cfg(test)]
mod test {
    use super::*;

    struct Parent {
        value: i32,
        child_a: Child,
        child_b: Child,
    }

    struct Child {
        value: i32,
    }


    impl State for Parent {

        fn value(&self) -> &dyn Any {
            &self.value
        }

        fn child<'a>(&'a self, name: &str) -> Option<&'a dyn Any> {
            match name {
                "child_a" => Some(&self.child_a),
                "child_b" => Some(&self.child_b),
                _ => None,
            }
        }

        fn child_mut<'a>(&'a mut self, name: &str) -> Option<&'a mut dyn Any> {
            match name {
                "child_a" => Some(&mut self.child_a),
                "child_b" => Some(&mut self.child_b),
                _ => None,
            }
        }
    }
    
    impl State for Child {

        fn value(&self) -> &dyn Any {
            &self.value
        }
    }
}
