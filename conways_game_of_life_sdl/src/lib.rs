pub use conways_game_of_life_impl::{config, BResult, Grid, Grid2d, LinearGrid};
use sdl2::{
    event::Event, keyboard::Keycode, pixels::Color, rect::Rect, render::WindowCanvas,
    video::WindowBuildError, EventPump, IntegerOrSdlError, Sdl, VideoSubsystem,
};
use std::{error::Error, fmt};

pub struct SDLInterface<G>
where
    G: Grid,
{
    _sdl: Sdl,
    _video: VideoSubsystem,
    canvas: WindowCanvas,
    event_pump: EventPump,
    grid: G,
}

pub type IResult<T> = Result<T, InterfaceError>;

#[derive(Debug)]
pub enum InterfaceError {
    IntegerOrSdlError(IntegerOrSdlError),
    String(String),
    WindowBuildError(WindowBuildError),
}

impl fmt::Display for InterfaceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for InterfaceError {}

impl From<String> for InterfaceError {
    fn from(string: String) -> Self {
        Self::String(string)
    }
}

impl From<WindowBuildError> for InterfaceError {
    fn from(wbe: WindowBuildError) -> Self {
        Self::WindowBuildError(wbe)
    }
}

impl From<IntegerOrSdlError> for InterfaceError {
    fn from(iose: IntegerOrSdlError) -> Self {
        Self::IntegerOrSdlError(iose)
    }
}

impl<'a, G> SDLInterface<G>
where
    G: Grid,
{
    pub fn new(window_name: &str, (w, h): (u32, u32), grid: G) -> IResult<Self> {
        let sdl = sdl2::init()?;
        let video = sdl.video()?;
        let window = video
            .window(window_name, w, h)
            .position_centered()
            .build()?;
        let canvas = window.into_canvas().build()?;
        let event_pump = sdl.event_pump()?;
        Ok(Self {
            _sdl: sdl,
            _video: video,
            canvas,
            event_pump,
            grid,
        })
    }

    pub fn tick(&mut self) -> IResult<bool> {
        let run = self.poll();
        self.render()?;
        self.grid.update();
        Ok(run)
    }

    fn poll(&mut self) -> bool {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    return false;
                }
                _ => (),
            }
        }
        true
    }

    fn render(&mut self) -> IResult<()> {
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();
        self.canvas.set_draw_color(Color::RGB(200, 200, 200));

        let window_size = self.canvas.window().size();
        let grid_size = self.grid.size();
        let cell_w = window_size.0 / grid_size.0;
        let cell_h = window_size.1 / grid_size.1;

        for y in 0..grid_size.1 {
            for x in 0..grid_size.0 {
                if self.grid.get_cell_unchecked((x, y)) {
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
