use gewy::{App, AppCtx, LogicalSize, WindowAttributes, run_app};

struct MyApp;
impl App for MyApp {
    fn start(&mut self, mut ctx: AppCtx) {
        log::info!("Started!!!");
        let attr = WindowAttributes::default()
            .with_title("Window")
            .with_inner_size(LogicalSize::new(512, 512));
        ctx.create_window(attr);
    }

    fn exit(&mut self, _ctx: AppCtx) {
        log::info!("Exiting!!!");
    }
}

fn main() {
    env_logger::init();
    run_app(MyApp);
}
