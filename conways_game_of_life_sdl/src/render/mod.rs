mod new_cell_color;

use crate::{input_pump::InputPump, Grid, GridPoint, GridUnit, IResult};
use new_cell_color::{CyclicalModulator, NewCellColorCyclical, NewCellColorHeatMap};
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
    DynamicHeatMap { hot: Rgb, cold: Rgb },
}

enum DrawOptionPrivate {
    Static(Color),
    DynamicCyclical(NewCellColorCyclical),
    DynamicHeatMap(NewCellColorHeatMap),
}

type Zoom = i32;

#[derive(Clone, Copy)]
pub struct Camera {
    x: f64,
    y: f64,
    zoom: Zoom,
    zoom_range: (Zoom, Zoom),
}

impl Camera {
    fn new(x: f64, y: f64, zoom: Zoom, mut zoom_range: (Zoom, Zoom)) -> Self {
        zoom_range.0 = zoom_range.0.max(1);
        Self {
            x,
            y,
            zoom,
            zoom_range,
        }
    }

    pub fn move_focus(&mut self, x: f64, y: f64) {
        self.x += x / self.zoom as f64;
        self.y += y / self.zoom as f64;
    }

    pub fn clamp(&mut self, x: &(f64, f64), y: &(f64, f64)) {
        self.x = self.x.clamp(x.0, x.1);
        self.y = self.y.clamp(y.0, y.1);
    }

    pub fn zoom(&mut self, zoom: Zoom) {
        self.zoom += zoom;
        self.zoom = self.zoom.clamp(self.zoom_range.0, self.zoom_range.1);
    }
}

pub enum CameraOpt {
    Centered,
    Position { x: f64, y: f64 },
}

enum RendererBuildStage {
    VideoSubsystem(VideoSubsystemStage),
    WindowBuilder(WindowBuilder),
    Window(Window),
    CanvasBuilder(CanvasBuilder),
    Canvas(WindowCanvas),
}

#[derive(Clone)]
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
    pub cursor_color: Color,
    pub draw_opt: DrawOption,
    pub camera_opt: CameraOpt,
    build_stage: RendererBuildStage,
    stage_commands: StageCommands,
}

macro_rules! set_command {
    ($fn_name:ident, $var_name:ident, $t:ty) => {
        pub fn $fn_name<F: FnMut($t) -> $t + 'static>(&mut self, f: F) -> &mut Self {
            self.stage_commands.$var_name = Some(Box::new(f));
            self
        }
    };
}

macro_rules! apply_command {
    ($self:ident, $var_name:ident, $conf_name:ident) => {
        if let Some(command) = &mut $self.stage_commands.$var_name {
            $conf_name = command($conf_name);
        }
    };
}

macro_rules! process_stages {
    ($self:ident, $grid_size:ident, $([$build_stage:ident, $var_name:ident, $next_stage:ident, $conf_name:ident, $ret:expr]),+, $(,)?) => {
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
		    let zoom = 1;
		    let zoom_range = (1, 20);
                    return Ok(Renderer {
			camera: match $self.camera_opt {
			    CameraOpt::Centered => {
				Camera::new(($grid_size.0/2) as f64, ($grid_size.1/2) as f64, zoom, zoom_range)
			    }
			    CameraOpt::Position {x, y} => Camera::new(x, y, zoom, zoom_range),
			},
			background_color: $self.background_color,
			cursor_color: $self.cursor_color,
			draw_opt: match $self.draw_opt {
			    DrawOption::Static(color) => DrawOptionPrivate::Static(color),
			    DrawOption::DynamicCyclical(rgb) => DrawOptionPrivate::DynamicCyclical(
				NewCellColorCyclical::new(CyclicalModulator::new(rgb), $grid_size),
			    ),
			    DrawOption::DynamicHeatMap { hot, cold } => {
				DrawOptionPrivate::DynamicHeatMap(NewCellColorHeatMap::new(hot, cold, $grid_size))
			    }
			},
                        _video: $self.video,
                        canvas,
                    });
                }
	}
	}
    }
}

impl RendererBuilder {
    pub fn new(sdl: &Sdl) -> IResult<Self> {
        Ok(Self {
            draw_opt: DrawOption::Static(Color::RGB(200, 200, 200)),
            background_color: Color::RGB(0, 0, 0),
            cursor_color: Color::RGB(255, 255, 255),
            video: sdl.video()?,
            camera_opt: CameraOpt::Centered,
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

    pub fn build(mut self, grid_size: GridPoint) -> IResult<Renderer> {
        process_stages!(
            self,
            grid_size,
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
    pub camera: Camera,
    pub background_color: Color,
    pub cursor_color: Color,
    _video: VideoSubsystem,
    canvas: WindowCanvas,
    draw_opt: DrawOptionPrivate,
}

impl Renderer {
    pub fn map_window_pos_to_cell(
        &self,
        (x, y): (i32, i32),
        grid_size: GridPoint,
    ) -> Option<GridPoint> {
        fn get_coord(
            v: i32,
            window_h: i32,
            zoom: f64,
            camera: f64,
            grid_size: GridUnit,
        ) -> Option<GridUnit> {
            let coord = (v - window_h) as f64 / zoom + camera;
            if coord >= 0. {
                let coord = coord as GridUnit;
                if coord < grid_size {
                    return Some(coord as GridUnit);
                }
            }
            None
        }

        let window_size = self.canvas.window().size();
        let window_h_w = window_size.0 as i32 / 2;
        let window_h_h = window_size.1 as i32 / 2;
        let zoom_f64 = self.camera.zoom as f64;

        Some((
            get_coord(x, window_h_w, zoom_f64, self.camera.x, grid_size.0)?,
            get_coord(y, window_h_h, zoom_f64, self.camera.y, grid_size.1)?,
        ))
    }

    pub fn render<G: Grid>(&mut self, grid: &G, input_pump: &InputPump) -> IResult<()> {
        self.canvas.set_draw_color(self.background_color);
        self.canvas.clear();
        if let DrawOptionPrivate::Static(cell_color) = self.draw_opt {
            self.canvas.set_draw_color(cell_color);
        }

        let fill_rect = {
            let window_size = self.canvas.window().size();
            let window_h_w = window_size.0 as i32 / 2;
            let window_h_h = window_size.1 as i32 / 2;

            let zoom_f64 = self.camera.zoom as f64;
            let zoom_u32 = self.camera.zoom as u32;

            move |canvas: &mut WindowCanvas,
                  x: GridUnit,
                  y: GridUnit,
                  camera_x: f64,
                  camera_y: f64|
                  -> Result<(), String> {
                canvas.fill_rect(Rect::new(
                    ((x as f64 - camera_x) * zoom_f64).ceil() as i32 + window_h_w,
                    ((y as f64 - camera_y) * zoom_f64).ceil() as i32 + window_h_h,
                    zoom_u32,
                    zoom_u32,
                ))
            }
        };

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
                fill_rect(&mut self.canvas, x, y, self.camera.x, self.camera.y)?;
            }
            Ok(())
        })?;

        if input_pump.mouse_in_window() {
            if let Some((x, y)) =
                self.map_window_pos_to_cell(input_pump.mouse().position(), grid.size())
            {
                self.canvas.set_draw_color(self.cursor_color);
                fill_rect(&mut self.canvas, x, y, self.camera.x, self.camera.y)?;
            }
        }

        self.canvas.present();
        Ok(())
    }

    pub fn update(&mut self) {
        match &mut self.draw_opt {
            DrawOptionPrivate::DynamicCyclical(ncc) => ncc.update(),
            DrawOptionPrivate::DynamicHeatMap(ncc) => ncc.update(),
            _ => (),
        }
    }

    pub fn reset(&mut self) {
        match &mut self.draw_opt {
            DrawOptionPrivate::DynamicCyclical(ncc) => ncc.reset(),
            DrawOptionPrivate::DynamicHeatMap(ncc) => ncc.reset(),
            _ => (),
        }
    }
}
