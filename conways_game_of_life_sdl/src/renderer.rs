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
    pub draw_opt: DrawOption,
    build_stage: RendererBuildStage,
    stage_commands: StageCommands,
}

#[derive(Clone, Copy)]
pub enum DrawOption {
    Static(Color),
    DynamicModulate(Rgb),
}

#[derive(Clone, Copy)]
pub enum Rgb {
    Red,
    Green,
    Blue,
}

impl Into<Color> for Rgb {
    fn into(self) -> Color {
	match self {
            Self::Red => Color::RGB(255, 0, 0),
            Self::Green => Color::RGB(0, 255, 0),
            Self::Blue => Color::RGB(0, 0, 255),
	}
    }
}

struct DynamicModulate {
    color_modulator: ColorModulator,
    cell_states: Vec<Vec<(bool, Color)>>,
}

impl DynamicModulate {
    fn init_cell_states<G: Grid>(&mut self, grid: &G) {
        let size = grid.size();
        self.cell_states =
            vec![vec![(false, self.color_modulator.color()); size.0 as usize]; size.1 as usize];
        grid.inspect(|(x, y), grid| {
            self.cell_states[y as usize][x as usize].0 = grid.get_cell_unchecked((x, y));
        });
    }
}

struct ColorModulator {
    color_state: Color,
    rgb: Rgb,
}

impl ColorModulator {
    fn new(rgb: Rgb) -> Self {
        Self {
            color_state: rgb.into(),
	    rgb,
        }
    }

    fn color(&self) -> Color {
        self.color_state
    }

    fn reset(&mut self) {
	self.color_state = self.rgb.into();
    }
    
    fn modulate(&mut self) {
        if self.color_state.r > 0 && self.color_state.b == 0 {
            self.color_state.r -= 1;
            self.color_state.g += 1;
        } else if self.color_state.g > 0 {
            self.color_state.g -= 1;
            self.color_state.b += 1;
        } else if self.color_state.b > 0 {
            self.color_state.b -= 1;
            self.color_state.r += 1;
        }
    }
}

enum DrawOptionPrivate {
    Static(Color),
    DynamicModulate(DynamicModulate),
}

impl Into<DrawOptionPrivate> for DrawOption {
    fn into(self) -> DrawOptionPrivate {
        match self {
            DrawOption::Static(color) => DrawOptionPrivate::Static(color),
            DrawOption::DynamicModulate(rgb) => {
                DrawOptionPrivate::DynamicModulate(DynamicModulate {
                    color_modulator: ColorModulator::new(rgb),
                    cell_states: Vec::new(),
                })
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
            DrawOptionPrivate::DynamicModulate(ref mut dynamic_latest) => {
                dynamic_latest.color_modulator.modulate();
                if dynamic_latest.cell_states.is_empty() {
                    dynamic_latest.init_cell_states(grid);
                }
            }
        }

        let window_size = self.canvas.window().size();
        let grid_size = grid.size();
        let cell_w = window_size.0 / grid_size.0 as u32;
        let cell_h = window_size.1 / grid_size.1 as u32;

        grid.try_inspect::<String, _>(|(x, y), grid| {
            let cell = grid.get_cell_unchecked((x, y));
            match &mut self.draw_opt {
                DrawOptionPrivate::DynamicModulate(dynamic_latest) => {
                    let cell_state = &mut dynamic_latest.cell_states[y as usize][x as usize];
                    if cell {
                        if !cell_state.0 {
                            cell_state.1 = dynamic_latest.color_modulator.color();
                            cell_state.0 = true;
                        }
                    } else {
                        cell_state.0 = false;
                    }
                    self.canvas.set_draw_color(cell_state.1);
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
            DrawOptionPrivate::DynamicModulate(dynamic_latest) => dynamic_latest.color_modulator.reset(),
	    _ => (),
	}
    }
}
