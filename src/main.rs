use gewy::{div, GewyApp, GewyWindowState, UIRenderer, Widget};
use gewy::layout::{Style, Dimension};

fn main() {
    env_logger::init();
    let mut app = GewyApp::new();
    app.add_window(GewyWindowState::new(512, 512, AppWidget));
    app.start();
}


struct AppWidget;
impl Widget for AppWidget {

    fn style(&self) -> Style {
        let mut style = Style::default();
        style.size.width = Dimension::Percent(1.0);
        style.size.height = Dimension::Percent(1.0);
        style
    }

    fn render(&self, r: &mut UIRenderer) {
        div(32.0, 32.0, r);
        div(32.0, 32.0, r);
        div(32.0, 32.0, r);
    }
}