use std::time::{Duration, Instant};

use druid::widget::{Button, Container, Flex, Label};
use druid::{
    AppLauncher, BoxConstraints, Color, Data, Env, Event, EventCtx, LayoutCtx, LifeCycle,
    LifeCycleCtx, LocalizedString, PaintCtx, Rect, Size, TimerToken, UpdateCtx, Widget, WidgetExt,
    WindowDesc,
};

use crate::server::Server;
pub struct Render {}
impl Render {
    pub fn new(server: Server) {
        let main_window = WindowDesc::new(ui_builder).window_size((600.0, 600.0));
        let data = MyData {
            server,
            ..Default::default()
        };
        AppLauncher::with_window(main_window)
            // .use_simple_logger()
            .launch(data)
            .unwrap();
    }
}
fn ui_builder() -> impl Widget<MyData> {
    // The label text will be computed dynamically based on the current locale and count
    let text = LocalizedString::new("hello-counter").with_arg("count", |data: &MyData, _env| {
        format!("{}", data.tps).into()
    });
    let label = Label::new(text).padding(5.0).center();

    let btnrow = Flex::column()
        .with_child(
            Button::new("Go faster")
                .on_click(|_ctx, data: &mut MyData, _env| {
                    data.tps = data.tps * 5.0;
                    if data.tps > 10000.0 {
                        data.tps = 10.0
                    }
                })
                .padding(5.0),
        )
        .with_child(
            Button::new("View pools")
                .on_click(|_ctx, data: &mut MyData, _env| {
                    data.highlight_pools = !data.highlight_pools;
                })
                .padding(5.0),
        );

    let container = Container::new(EvoWidget::new()).fix_size(500.0, 500.0);

    Flex::column()
        .with_child(label)
        .with_child(btnrow)
        .with_child(container)
}
const FPS: usize = 30;
#[derive(Clone, Data)]
struct MyData {
    #[data(ignore)]
    pub server: Server,
    pub tps: f32,
    pub i: f32,
    pub highlight_pools: bool,
}
impl Default for MyData {
    fn default() -> Self {
        Self {
            tps: 10.0,
            server: Server::default(),
            i: 0.0,
            highlight_pools: true,
        }
    }
}

struct EvoWidget {
    timer_id: TimerToken,
    last_update: Instant,
}
impl EvoWidget {
    pub fn new() -> Self {
        Self {
            timer_id: TimerToken::INVALID,
            last_update: Instant::now(),
        }
    }
    pub fn iter_interval(&self, data: &MyData) -> u64 {
        (1000 / FPS.min(data.tps as usize)) as u64
    }
}

impl Widget<MyData> for EvoWidget {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut MyData, _env: &Env) {
        match event {
            Event::WindowConnected => {
                let deadline = Duration::from_millis(self.iter_interval(data));
                self.last_update = Instant::now();
                self.timer_id = ctx.request_timer(deadline);
            }
            Event::Timer(id) => {
                if *id == self.timer_id {
                    let tps = data.tps;
                    let fps = FPS as f32;
                    let runs = (tps / fps).round().max(1.0) as usize;
                    // println!("run {} {} ", fps, tps);
                    for _i in 0..runs {
                        data.server.tick();
                    }
                    ctx.request_paint();
                    let deadline = Duration::from_millis(self.iter_interval(data));
                    self.last_update = Instant::now();
                    self.timer_id = ctx.request_timer(deadline);
                }
            }
            _ => {}
        }
    }

    fn lifecycle(
        &mut self,
        _ctx: &mut LifeCycleCtx,
        _event: &LifeCycle,
        _data: &MyData,
        _env: &Env,
    ) {
    }

    fn update(&mut self, ctx: &mut UpdateCtx, _old_data: &MyData, data: &MyData, _env: &Env) {
        if (data.tps - data.tps).abs() > 0.001 {
            let deadline = Duration::from_millis(self.iter_interval(data))
                .checked_sub(Instant::now().duration_since(self.last_update))
                .unwrap_or_else(|| Duration::from_secs(0));
            self.timer_id = ctx.request_timer(deadline);
        }
    }

    fn layout(
        &mut self,
        _layout_ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        _data: &MyData,
        _env: &Env,
    ) -> Size {
        let max_size = bc.max();
        let min_side = max_size.height.min(max_size.width);
        Size {
            width: 500.0,
            height: 500.0,
        }
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &MyData, _env: &Env) {
        use druid::RenderContext;

        let bounds = ctx.size().to_rect();
        if data.tps > 40.0 {
            ctx.fill(bounds, &Color::BLACK);
        } else {
            ctx.fill(bounds, &Color::BLACK);
        }
        data.server.sim.replicants.iter().for_each(|rep| {
            let s = bounds.width() as f64 / data.server.sim.world.width as f64;
            let x0 = s * rep.pos.0 as f64;
            let x1 = x0 + s;
            let y0 = s * rep.pos.1 as f64;
            let y1 = y0 + s;
            let is_alive = rep.is_alive(&data.server.sim.world, &data.server.sim.mapper);
            // let h = rep.net.links().count() * 30 % 360;
            let c = rep.net.color;
            let max = c[0].max(c[1]).max(c[2]) as f64;
            let min = c[0].min(c[1]).min(c[2]) as f64;
            let diff = (max - min).max(0.01);
            let base = 0.2;
            let opacity = if is_alive { 1.0 } else { 0.4 };
            let color = if data.highlight_pools {
                if rep.net.pool() == 0 {
                    Color::rgba(1.0, 0.3, 0.1, opacity)
                } else if rep.net.pool() == 1 {
                    Color::rgba(0.3, 1.0, 0.1, opacity)
                } else {
                    Color::rgba(0.2, 0.4, 1.0, opacity)
                }
            } else {
                Color::rgba(
                    base + (1.0 - base) * (-min + (c[0] as f64) / diff),
                    base + (1.0 - base) * (-min + (c[1] as f64) / diff),
                    base + (1.0 - base) * (-min + (c[2] as f64) / diff),
                    opacity,
                )
            };
            // let color = if is_alive {
            //     let base = 0.2;
            //     Color::rgb(
            //         base + (1.0 - base) * (-min + (c[0] as f64) / diff),
            //         base + (1.0 - base) * (-min + (c[1] as f64) / diff),
            //         base + (1.0 - base) * (-min + (c[2] as f64) / diff),
            //     )
            //     // Color::hlc(h as f64, 80.0, 120.0)
            //     // color
            //     // Color::rgb(0.3, 1.0, 0.1)
            // } else {
            //     let base = 0.05;
            //     let top = 0.9;
            //     Color::rgb(
            //         base + (1.0 - base - top) * (-min + (c[0] as f64) / diff),
            //         base + (1.0 - base - top) * (-min + (c[1] as f64) / diff),
            //         base + (1.0 - base - top) * (-min + (c[2] as f64) / diff),
            //     )
            //     // Color::RED
            //     // Color::rgb(1.0, 0.6, 0.0)
            //     // Color::hlc(h as f64, 70.0, 110.0)
            // };
            ctx.fill(Rect::new(x0, y0, x1, y1), &color);
        });
    }
}
