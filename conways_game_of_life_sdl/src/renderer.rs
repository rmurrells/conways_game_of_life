use crate::{Grid, IResult};
use sdl2::{
    pixels::Color,
    rect::Rect,
    render::{CanvasBuilder, WindowCanvas},
    video::{Window, WindowBuilder},
    Sdl, VideoSubsystem,
};

pub struct RendererBuilder {
    pub video: VideoSubsystem,
    pub background_color: Color,
    pub cell_color: Color,
    build_stage: RendererBuildStage,
    stage_commands: StageCommands,
}

pub enum RendererBuildStage {
    VideoSubsystem(VideoSubsystemStage),
    WindowBuilder(WindowBuilder),
    Window(Window),
    CanvasBuilder(CanvasBuilder),
    Canvas(WindowCanvas),
}

pub struct VideoSubsystemStage {
    pub window_name: String,
    pub window_size: (u32, u32),
}

struct StageCommands {
    video_subsystem: Option<Box<dyn FnMut(&mut VideoSubsystemStage)>>,
    window_builder: Option<Box<dyn FnMut(&mut WindowBuilder)>>,
    window: Option<Box<dyn FnMut(&mut Window)>>,
    canvas_builder: Option<Box<dyn FnMut(&mut CanvasBuilder)>>,
    canvas: Option<Box<dyn FnMut(&mut WindowCanvas)>>,
}

impl StageCommands {
    fn new() -> Self {
        Self {
            video_subsystem: None,
            window_builder: None,
            window: None,
            canvas_builder: None,
            canvas: None,
        }
    }
}

impl RendererBuilder {
    pub fn new(sdl: &Sdl) -> IResult<Self> {
        Ok(Self {
            video: sdl.video()?,
            background_color: Color::RGB(0, 0, 0),
            cell_color: Color::RGB(200, 200, 200),
            build_stage: RendererBuildStage::VideoSubsystem(VideoSubsystemStage {
                window_name: "conways_game_of_life".into(),
                window_size: (800, 600),
            }),
            stage_commands: StageCommands::new(),
        })
    }

    pub fn video_subsystem_command<F: FnMut(&mut VideoSubsystemStage) + 'static>(&mut self, f: F) {
        self.stage_commands.video_subsystem = Some(Box::new(f));
    }

    pub fn window_builder_command<F: FnMut(&mut WindowBuilder) + 'static>(&mut self, f: F) {
        self.stage_commands.window_builder = Some(Box::new(f));
    }

    pub fn window_command<F: FnMut(&mut Window) + 'static>(&mut self, f: F) {
        self.stage_commands.window = Some(Box::new(f));
    }

    pub fn canvas_builder_command<F: FnMut(&mut CanvasBuilder) + 'static>(&mut self, f: F) {
        self.stage_commands.canvas_builder = Some(Box::new(f));
    }

    pub fn canvas_command<F: FnMut(&mut WindowCanvas) + 'static>(&mut self, f: F) {
        self.stage_commands.canvas = Some(Box::new(f));
    }

    pub fn build(mut self) -> IResult<Renderer> {
        loop {
            self.build_stage = match self.build_stage {
                RendererBuildStage::VideoSubsystem(mut vss) => {
                    if let Some(command) = &mut self.stage_commands.video_subsystem {
                        command(&mut vss);
                    }
                    RendererBuildStage::WindowBuilder(self.video.window(
                        &vss.window_name,
                        vss.window_size.0,
                        vss.window_size.1,
                    ))
                }
                RendererBuildStage::WindowBuilder(mut wb) => {
                    if let Some(command) = &mut self.stage_commands.window_builder {
                        command(&mut wb);
                    }
                    RendererBuildStage::Window(wb.build()?)
                }
                RendererBuildStage::Window(mut w) => {
                    if let Some(command) = &mut self.stage_commands.window {
                        command(&mut w);
                    }
                    RendererBuildStage::CanvasBuilder(w.into_canvas())
                }
                RendererBuildStage::CanvasBuilder(mut cb) => {
                    if let Some(command) = &mut self.stage_commands.canvas_builder {
                        command(&mut cb);
                    }
                    RendererBuildStage::Canvas(cb.build()?)
                }
                RendererBuildStage::Canvas(mut canvas) => {
                    if let Some(command) = &mut self.stage_commands.canvas {
                        command(&mut canvas);
                    }
                    return Ok(Renderer {
                        _video: self.video,
                        canvas,
                        background_color: self.background_color,
                        cell_color: self.cell_color,
                    });
                }
            }
        }
    }
}

pub struct Renderer {
    _video: VideoSubsystem,
    canvas: WindowCanvas,
    pub background_color: Color,
    pub cell_color: Color,
}

impl Renderer {
    pub fn render<G: Grid>(&mut self, grid: &G) -> IResult<()> {
        self.canvas.set_draw_color(self.background_color);
        self.canvas.clear();
        self.canvas.set_draw_color(self.cell_color);

        let window_size = self.canvas.window().size();
        let grid_size = grid.size();
        let cell_w = window_size.0 / grid_size.0;
        let cell_h = window_size.1 / grid_size.1;

        for y in 0..grid_size.1 {
            for x in 0..grid_size.0 {
                if grid.get_cell_unchecked((x, y)) {
                    self.canvas.fill_rect(Rect::new(
                        (x * cell_w) as i32,
                        (y * cell_h) as i32,
                        cell_w,
                        cell_h,
                    ))?;
                }
            }
        }
        self.canvas.present();
        Ok(())
    }
}
