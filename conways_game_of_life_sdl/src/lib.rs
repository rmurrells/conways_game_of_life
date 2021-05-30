mod input_pump;
pub mod renderer;

pub use conways_game_of_life_impl::{
    config, BResult, Grid, Grid1dVec, Grid2dArr, Grid2dVec, GridPoint, GridUnit,
};
use input_pump::{Input, InputPump};
use renderer::{Renderer, RendererBuilder};
use sdl2::{video::WindowBuildError, IntegerOrSdlError, Sdl};
use std::{error::Error, fmt, marker::PhantomData};

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

pub struct SDLInterfaceBuilder<G>
where
    G: Grid,
{
    pub sdl: Sdl,
    pub renderer_builder: RendererBuilder,
    input_pump: InputPump,
    phantom: PhantomData<G>,
}

impl<G> SDLInterfaceBuilder<G>
where
    G: Clone + Grid,
{
    pub fn new() -> IResult<Self> {
        let sdl = sdl2::init()?;
        let renderer_builder = RendererBuilder::new(&sdl)?;
        Ok(Self {
            input_pump: InputPump::new(&sdl)?,
            sdl,
            renderer_builder,
            phantom: PhantomData,
        })
    }

    pub fn build(self, grid: G) -> IResult<SDLInterface<G>> {
        Ok(SDLInterface::<G> {
            renderer: self.renderer_builder.build(grid.size())?,
            input_pump: self.input_pump,
            _sdl: self.sdl,
            init_grid: grid.clone(),
            grid,
            pause: false,
        })
    }
}

pub struct SDLInterface<G>
where
    G: Grid,
{
    _sdl: Sdl,
    renderer: Renderer,
    input_pump: InputPump,
    init_grid: G,
    grid: G,
    pause: bool,
}

impl<'a, G> SDLInterface<G>
where
    G: Clone + Grid,
{
    pub fn run(&mut self) -> IResult<()> {
        while self.tick()? {}
        Ok(())
    }

    pub fn tick(&mut self) -> IResult<bool> {
        let mut run = true;
        for input in self.input_pump.poll_iter() {
            match input {
                Input::MoveCamera { x, y } => {
                    self.renderer.camera.move_focus(x as f64, y as f64);
                    let grid_size = self.grid.size();
                    self.renderer
                        .camera
                        .clamp(&(0., grid_size.0 as f64), &(0., grid_size.1 as f64));
                }
                Input::ZoomCamera { zoom } => self.renderer.camera.zoom(zoom.signum()),
                Input::Pause => self.pause = !self.pause,
                Input::Run => (),
                Input::Reset => {
                    self.renderer.reset();
                    self.grid = self.init_grid.clone();
                }
                Input::Quit => run = false,
            }
        }

        self.renderer.render(&self.grid)?;
        if !self.pause {
            self.grid.update();
            self.renderer.update();
        }
        Ok(run)
    }
}
