use winit::event::{ElementState, MouseButton as WinitMouseButton};

/// All events with regards to input.
pub enum InputEvent {
    MousePressed { button: MouseButton },
    MouseReleased { button: MouseButton },
    CursorEntered,
    CursorLeft,
    CursorMoved { x: f32, y: f32 },
}

impl InputEvent {

    pub(crate) fn from_winit_mouse(state: ElementState, winit_button: WinitMouseButton) -> Self {
        let button = MouseButton::from_winit(winit_button);
        match state {
            ElementState::Pressed   => Self::MousePressed { button },
            ElementState::Released  => Self::MouseReleased { button },
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    Back,
    Forward,
    Other(u16),
}

impl MouseButton {
    fn from_winit(mouse_button: WinitMouseButton) -> Self {
        match mouse_button {
            WinitMouseButton::Left          => Self::Left,
            WinitMouseButton::Right         => Self::Right,
            WinitMouseButton::Middle        => Self::Middle,
            WinitMouseButton::Back          => Self::Back,
            WinitMouseButton::Forward       => Self::Forward,
            WinitMouseButton::Other(num)    => Self::Other(num),
        }
    }
}