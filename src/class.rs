/**
 * Anything that can manipulate a widget.
 * This is usually either a callback function, or a tuple of callback functions.
 * This is useful for externalizing widget configuration.
 */
pub trait Class<W> {
    fn apply(&self, widget: &mut W);
    fn to_widget(&self) -> W
    where
        W: Default
    {
        let mut widget = W::default();
        self.apply(&mut widget);
        widget
    }
}

impl<W, F: Fn(&mut W)> Class<W> for F {
    fn apply(&self, widget: &mut W) {
        self(widget);
    }
}

impl<W> Class<W> for () {
    fn apply(&self, _widget: &mut W) {}
}

impl<W, C0: Class<W>> Class<W> for (C0,) {
    fn apply(&self, widget: &mut W) {
        self.0.apply(widget);
    }
}

impl<W, C0: Class<W>, C1: Class<W>> Class<W> for (C0, C1) {
    fn apply(&self, widget: &mut W) {
        self.0.apply(widget);
        self.1.apply(widget);
    }
}

impl<W, C0: Class<W>, C1: Class<W>, C2: Class<W>> Class<W> for (C0, C1, C2) {
    fn apply(&self, widget: &mut W) {
        self.0.apply(widget);
        self.1.apply(widget);
        self.2.apply(widget);
    }
}

impl<W, C0: Class<W>, C1: Class<W>, C2: Class<W>, C3: Class<W>> Class<W> for (C0, C1, C2, C3) {
    fn apply(&self, widget: &mut W) {
        self.0.apply(widget);
        self.1.apply(widget);
        self.2.apply(widget);
        self.3.apply(widget);
    }
}

impl<
    W,
    C0: Class<W>,
    C1: Class<W>,
    C2: Class<W>,
    C3: Class<W>,
    C4: Class<W>,
> Class<W> for (C0, C1, C2, C3, C4) {
    fn apply(&self, widget: &mut W) {
        self.0.apply(widget);
        self.1.apply(widget);
        self.2.apply(widget);
        self.3.apply(widget);
        self.4.apply(widget);
    }
}

impl<
    W,
    C0: Class<W>,
    C1: Class<W>,
    C2: Class<W>,
    C3: Class<W>,
    C4: Class<W>,
    C5: Class<W>,
> Class<W> for (C0, C1, C2, C3, C4, C5) {
    fn apply(&self, widget: &mut W) {
        self.0.apply(widget);
        self.1.apply(widget);
        self.2.apply(widget);
        self.3.apply(widget);
        self.4.apply(widget);
        self.5.apply(widget);
    }
}

impl<
    W, 
    C0: Class<W>,
    C1: Class<W>,
    C2: Class<W>,
    C3: Class<W>,
    C4: Class<W>,
    C5: Class<W>,
    C6: Class<W>,
> Class<W> for (C0, C1, C2, C3, C4, C5, C6) {
    fn apply(&self, widget: &mut W) {
        self.0.apply(widget);
        self.1.apply(widget);
        self.2.apply(widget);
        self.3.apply(widget);
        self.4.apply(widget);
        self.5.apply(widget);
        self.6.apply(widget);
    }
}

impl<
    W,
    C0: Class<W>,
    C1: Class<W>,
    C2: Class<W>,
    C3: Class<W>,
    C4: Class<W>,
    C5: Class<W>,
    C6: Class<W>,
    C7: Class<W>,
> Class<W> for (C0, C1, C2, C3, C4, C5, C6, C7) {
    fn apply(&self, widget: &mut W) {
        self.0.apply(widget);
        self.1.apply(widget);
        self.2.apply(widget);
        self.3.apply(widget);
        self.4.apply(widget);
        self.5.apply(widget);
        self.6.apply(widget);
    }
}