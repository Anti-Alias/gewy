use dyn_clone::DynClone;
use downcast_rs::{Downcast, impl_downcast};

/// A message emitted by either a [`Widget`](crate::Widget) or a [`State`](crate::State).
pub trait Message: Downcast + DynClone + 'static {}
impl<T: Downcast + DynClone + 'static> Message for T {}
impl_downcast!(Message);
dyn_clone::clone_trait_object!(Message);


/// Dynamic form of a [`Message`].
#[derive(Clone)]
pub struct DynMessage(Box<dyn Message>);

impl DynMessage {
    pub fn new(msg: impl Message) -> Self {
        Self(Box::new(msg))
    }
    pub fn as_ref(&self) -> &dyn Message {
        self.0.as_ref()
    }
    pub fn downcast_ref<M: Message>(&self) -> Option<&M> {
        self.0.downcast_ref()
    }
}


/// Maps an input message to an output message.
pub trait Mapper: 'static {
    fn map(&self, msg: &dyn Message) -> Option<DynMessage>;
}

impl Mapper for () {
    fn map(&self, _msg: &dyn Message) -> Option<DynMessage> { None }
}

impl<I, O> Mapper for (I, O)
where
    I: Message + PartialEq,
    O: Message + Clone,
{
    fn map(&self, message: &dyn Message) -> Option<DynMessage> {
        let Some(input) = message.downcast_ref::<I>() else { return None };
        if input == &self.0 {
            let output = DynMessage::new(self.1.clone());
            Some(output)
        }
        else {
            None
        }
    }
}

impl<M: Mapper, const N: usize> Mapper for [M; N] {
    fn map(&self, input: &dyn Message) -> Option<DynMessage> {
        for mapper in self {
            if let Some(output) = mapper.map(input) {
                return Some(output);
            }
        }
        None
    }
}


/// Dynamic form of [`Mapper`].
pub struct DynMapper(Box<dyn Mapper>);

impl DynMapper {
    #[inline(always)]
    pub fn map(&self, input: &dyn Message) -> Option<DynMessage> {
        self.0.map(input)
    }
}

impl<M: Mapper> From<M> for DynMapper {
    fn from(mapper: M) -> Self {
        Self(Box::new(mapper))
    }
}

impl Default for DynMapper {
    fn default() -> Self {
        Self(Box::new(()))
    }
}