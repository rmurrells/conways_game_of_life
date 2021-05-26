mod renderer;

pub use conways_game_of_life_impl::{config, BResult, Grid, Grid2d, LinearGrid};
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
            grid,
            renderer: self.renderer_builder.build()?,
            event_pump: self.sdl.event_pump()?,
            _sdl: self.sdl,
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
    grid: G,
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
}