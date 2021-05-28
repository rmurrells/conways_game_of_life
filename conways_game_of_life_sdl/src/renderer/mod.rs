mod new_cell_color;

use crate::{Grid, IResult};
use new_cell_color::{CyclicalModulator, NewCellColor, NewCellColorCyclical, NewCellColorHeatMap};
pub use new_cell_color::{CyclicalModulatorOpt, Rgb, Rygcbm};
use sdl2::{
    pixels::Color,
    rect::Rect,
    render::{CanvasBuilder, WindowCanvas},
    video::{Window, WindowBuilder},
    Sdl, VideoSubsystem,
};

#[derive(Clone, Copy)]
pub enum DrawOption {
    Static(Color),
    DynamicCyclical(CyclicalModulatorOpt),
    DynamicHeatMap(Rgb, Rgb),
}

enum DrawOptionPrivate {
    Static(Color),
    DynamicCyclical(NewCellColorCyclical),
    DynamicHeatMap(NewCellColorHeatMap),
}

impl From<DrawOption> for DrawOptionPrivate {
    fn from(draw_option: DrawOption) -> DrawOptionPrivate {
        match draw_option {
            DrawOption::Static(color) => DrawOptionPrivate::Static(color),
            DrawOption::DynamicCyclical(rgb) => DrawOptionPrivate::DynamicCyclical(
                NewCellColorCyclical::new(CyclicalModulator::new(rgb)),
            ),
            DrawOption::DynamicHeatMap(hot, cold) => {
                DrawOptionPrivate::DynamicHeatMap(NewCellColorHeatMap::new(hot, cold))
            }
        }
    }
}

enum RendererBuildStage {
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

pub struct RendererBuilder {
    pub video: VideoSubsystem,
    pub background_color: Color,
    pub draw_opt: DrawOption,
    build_stage: RendererBuildStage,
    stage_commands: StageCommands,
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
			draw_opt: $self.draw_opt.into(),
			background_color: $self.background_color,
                        _video: $self.video,
                        canvas,
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
            draw_opt: DrawOption::Static(Color::RGB(200, 200, 200)),
            background_color: Color::RGB(0, 0, 0),
            video: sdl.video()?,
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
    pub background_color: Color,
    _video: VideoSubsystem,
    canvas: WindowCanvas,
    draw_opt: DrawOptionPrivate,
}

impl Renderer {
    pub fn render<G: Grid>(&mut self, grid: &G) -> IResult<()> {
        self.canvas.set_draw_color(self.background_color);
        self.canvas.clear();
        match self.draw_opt {
            DrawOptionPrivate::Static(cell_color) => self.canvas.set_draw_color(cell_color),
            DrawOptionPrivate::DynamicCyclical(ref mut ncc) => ncc.update(grid),
            DrawOptionPrivate::DynamicHeatMap(ref mut ncc) => ncc.update(grid),
        }

        let window_size = self.canvas.window().size();
        let grid_size = grid.size();
        let cell_w = window_size.0 / grid_size.0 as u32;
        let cell_h = window_size.1 / grid_size.1 as u32;

        grid.try_inspect::<String, _>(|(x, y), grid| {
            let cell = grid.get_cell_unchecked((x, y));
            match &mut self.draw_opt {
                DrawOptionPrivate::DynamicCyclical(ncc) => {
                    self.canvas.set_draw_color(ncc.get_cell_color((x, y), cell));
                }
                DrawOptionPrivate::DynamicHeatMap(ncc) => {
                    self.canvas.set_draw_color(ncc.get_cell_color((x, y), cell));
                }
                _ => (),
            }
            if cell {
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

    pub fn reset(&mut self) {
        match &mut self.draw_opt {
            DrawOptionPrivate::DynamicCyclical(ncc) => ncc.reset(),
            DrawOptionPrivate::DynamicHeatMap(ncc) => ncc.reset(),
            _ => (),
        }
    }
}
