mod renderer;

pub use conways_game_of_life_impl::{
    config, BResult, Grid, Grid1dVec, Grid2dArr, Grid2dVec, GridUnit,
};
use renderer::Renderer;
pub use renderer::{RendererBuildStage, RendererBuilder};
use sdl2::{
    event::Event, keyboard::Keycode, video::WindowBuildError, EventPump, IntegerOrSdlError, Sdl,
};
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
    phantom: PhantomData<G>,
}

impl<G> SDLInterfaceBuilder<G>
where
    G: Grid,
{
    pub fn new() -> IResult<Self> {
        let sdl = sdl2::init()?;
        let renderer_builder = RendererBuilder::new(&sdl)?;
        Ok(Self {
            sdl,
            renderer_builder,
            phantom: PhantomData,
        })
    }

    pub fn build(self, grid: G) -> IResult<SDLInterface<G>> {
        Ok(SDLInterface::<G> {
            renderer: self.renderer_builder.build()?,
            event_pump: self.sdl.event_pump()?,
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
    event_pump: EventPump,
    init_grid: G,
    grid: G,
    pause: bool,
}

impl<'a, G> SDLInterface<G>
where
    G: Grid,
{
    pub fn run(&mut self) -> IResult<()> {
        while self.tick()? {}
        Ok(())
    }

    pub fn tick(&mut self) -> IResult<bool> {
        let run = self.poll();
        self.renderer.render(&self.grid)?;
        if !self.pause {
            self.grid.update();
        }
        Ok(run)
    }

    fn poll(&mut self) -> bool {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyUp {
                    keycode: Some(Keycode::Escape),
                    ..
                } => return false,
                Event::KeyUp {
                    keycode: Some(key), ..
                } => match key {
                    Keycode::R => self.grid = self.init_grid.clone(),
                    Keycode::Space => self.pause = !self.pause,
                    _ => (),
                },
                _ => (),
            }
        }
        true
    }
}
