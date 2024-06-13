use gewy::{GewyApp, GewyWindowState};

fn main() {
    env_logger::init();
    let mut app = GewyApp::new();
    app.add_window(GewyWindowState::default());
    app.add_window(GewyWindowState::default());
    app.start();
}
