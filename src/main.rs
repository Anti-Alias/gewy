use gewy::{App, AppCtx, LogicalSize, State, WindowAttributes, run_app};

struct Handler;
impl App for Handler {
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
    run_app(Handler);
}

#[derive(State)]
struct Dog {
    #[state]
    sub_a: SubState,
    #[state]
    sub_b: SubState,
    #[state]
    sub_c: SubState,
}


#[derive(State)]
struct SubState {
    value: i32,
}
