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

type CommandOpt<T> = Option<Box<dyn FnMut(T) -> T>>;

struct StageCommands {
    video_subsystem: CommandOpt<VideoSubsystemStage>,
    window_builder: CommandOpt<WindowBuilder>,
    window: CommandOpt<Window>,
    canvas_builder: CommandOpt<CanvasBuilder>,
    canvas: CommandOpt<WindowCanvas>,
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

macro_rules! set_command {
    ($fn_name:ident, $var_name:ident, $t:ty) => {
        pub fn $fn_name<F: FnMut($t) -> $t + 'static>(&mut self, f: F) {
            self.stage_commands.$var_name = Some(Box::new(f));
        }
    };
}

macro_rules! process_stages {
    ($self:ident, $([$build_stage:ident, $var_name:ident, $next_stage:ident, $conf_name:ident, $ret:expr]),+, $(,)?) => {
        loop {
	$self.build_stage = match $self.build_stage {
	    $(
		RendererBuildStage::$build_stage(mut $conf_name) => {
		    apply_command!($self, $var_name, $conf_name);
		    RendererBuildStage::$next_stage($ret)
		}
	    )+
	        RendererBuildStage::Canvas(mut canvas) => {
		    apply_command!($self, canvas, canvas);
                    return Ok(Renderer {
                        _video: $self.video,
                           canvas,
                        background_color: $self.background_color,
                        cell_color: $self.cell_color,
                    });
                }
	}
	}
    }
}

macro_rules! apply_command {
    ($self:ident, $var_name:ident, $conf_name:ident) => {
        if let Some(command) = &mut $self.stage_commands.$var_name {
            $conf_name = command($conf_name);
        }
    };
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

    set_command!(
        video_subsystem_command,
        video_subsystem,
        VideoSubsystemStage
    );
    set_command!(window_builder_command, window_builder, WindowBuilder);
    set_command!(window_command, window, Window);
    set_command!(canvas_builder_command, canvas_builder, CanvasBuilder);
    set_command!(canvas_command, canvas, WindowCanvas);

    pub fn build(mut self) -> IResult<Renderer> {
        process_stages!(
            self,
            [
                VideoSubsystem,
                video_subsystem,
                WindowBuilder,
                vss,
                self.video
                    .window(&vss.window_name, vss.window_size.0, vss.window_size.1,)
            ],
            [WindowBuilder, window_builder, Window, wb, wb.build()?],
            [Window, window, CanvasBuilder, w, w.into_canvas()],
            [CanvasBuilder, canvas_builder, Canvas, cb, cb.build()?],
        );
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
        let cell_w = window_size.0 / grid_size.0 as u32;
        let cell_h = window_size.1 / grid_size.1 as u32;

        grid.try_inspect::<String, _>(|(x, y), grid| {
            if grid.get_cell_unchecked((x, y)) {
                self.canvas.fill_rect(Rect::new(
                    (x as u32 * cell_w) as i32,
                    (y as u32 * cell_h) as i32,
                    cell_w,
                    cell_h,
                ))?;
            }
            Ok(())
        })?;

        self.canvas.present();
        Ok(())
    }
}
