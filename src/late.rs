/// An [`Option`]-like enum.
/// Used to convey data that is initialized at a later time.
/// Used heavily on fields in [`Widget`](crate::Widget)s.
#[derive(Copy, Clone, Eq, PartialEq, Default, Debug, Ord, PartialOrd, Hash)]
pub enum Late<T> {
    /// Field is uninitialized
    #[default]
    Uninit,
    /// Field is initialized
    Init(T),
}

impl<T> Late<T> {
    pub fn unwrap(self) -> T {
        match self {
            Late::Uninit => panic!("called `Late::unwrap()` on an `Uninit` value"),
            Late::Init(value) => value,
        }
    }

    pub fn is_init(&self) -> bool {
        match self {
            Late::Uninit => false,
            Late::Init(_) => true,
        }
    }

    pub fn is_uninit(&self) -> bool {
        match self {
            Late::Uninit => true,
            Late::Init(_) => false,
        }
    }

    pub fn as_ref(&self) -> Late<&T> {
        match self {
            Late::Uninit => Late::Uninit,
            Late::Init(value) => Late::Init(value),
        }
    }

    pub fn as_mut(&mut self) -> Late<&mut T> {
        match self {
            Late::Uninit => Late::Uninit,
            Late::Init(value) => Late::Init(value),
        }
    }

    pub fn to_option(self) -> Option<T>
    where 
        T: Copy,
    {
        match self {
            Late::Uninit => None,
            Late::Init(value) => Some(value),
        }
    }
}

impl<T> From<Option<T>> for Late<T> {
    fn from(value: Option<T>) -> Self {
        match value {
            Some(value) => Self::Init(value),
            None => Self::Uninit,
        }
    }
}