use crate::{BResult, Grid, GridPoint, IResult};
use sdl2::{
    event::Event,
    keyboard::Keycode,
    mouse::{MouseButton, MouseState},
    EventPump, Sdl,
};

pub struct Mouse {
    x: i32,
    y: i32,
    left: bool,
    right: bool,
}

impl Mouse {
    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }
}

impl From<MouseState> for Mouse {
    fn from(mouse_state: MouseState) -> Self {
        Self {
            x: mouse_state.x(),
            y: mouse_state.y(),
            left: mouse_state.left(),
            right: mouse_state.right(),
        }
    }
}

pub struct InputPump {
    event_pump: EventPump,
    mouse: Mouse,
    draw_state: Option<bool>,
}

pub enum Input {
    DrawCell { point: (i32, i32) },
    MoveCamera { x: i32, y: i32 },
    OneFrame,
    Pause,
    Quit,
    Reset,
    Run,
    ZoomCamera { zoom: i32 },
}

impl InputPump {
    pub fn new(sdl: &Sdl) -> IResult<Self> {
        let event_pump = sdl.event_pump()?;
        Ok(Self {
            mouse: event_pump.mouse_state().into(),
            event_pump,
            draw_state: None,
        })
    }

    pub fn poll_event(&mut self) -> Option<Input> {
        Some(match self.event_pump.poll_event()? {
            Event::Quit { .. }
            | Event::KeyUp {
                keycode: Some(Keycode::Escape),
                ..
            } => Input::Quit,
            Event::KeyDown {
                keycode: Some(key), ..
            } => match key {
                Keycode::Equals => Input::ZoomCamera { zoom: 1 },
                Keycode::Minus => Input::ZoomCamera { zoom: -1 },
                Keycode::Return => Input::OneFrame,
                _ => Input::Run,
            },
            Event::KeyUp {
                keycode: Some(key), ..
            } => match key {
                Keycode::R => Input::Reset,
                Keycode::Space => Input::Pause,
                _ => Input::Run,
            },
            Event::MouseButtonDown { mouse_btn, .. } => match mouse_btn {
                MouseButton::Left => {
                    self.mouse.left = true;
                    Input::Run
                }
                MouseButton::Right => {
                    self.mouse.right = true;
                    Input::DrawCell {
                        point: (self.mouse.x, self.mouse.y),
                    }
                }
                _ => Input::Run,
            },
            Event::MouseButtonUp { mouse_btn, .. } => {
                match mouse_btn {
                    MouseButton::Left => self.mouse.left = false,
                    MouseButton::Right => {
                        self.mouse.right = false;
                        self.draw_state = None;
                    }
                    _ => (),
                }
                Input::Run
            }
            Event::MouseMotion {
                x, y, xrel, yrel, ..
            } => {
                self.mouse.x = x;
                self.mouse.y = y;
                if self.mouse.left {
                    Input::MoveCamera { x: -xrel, y: -yrel }
                } else if self.mouse.right {
                    Input::DrawCell {
                        point: (self.mouse.x, self.mouse.y),
                    }
                } else {
                    Input::Run
                }
            }
            Event::MouseWheel { y, .. } => Input::ZoomCamera { zoom: y },
            _ => Input::Run,
        })
    }

    pub fn mouse(&self) -> &Mouse {
        &self.mouse
    }

    pub fn draw<G: Grid>(&mut self, grid: &mut G, point: GridPoint) -> BResult<()> {
        let state = if let Some(state) = self.draw_state {
            state
        } else {
            let state = !grid.get_cell_unchecked(point);
            self.draw_state = Some(state);
            state
        };
        grid.set_cell(point, state)
    }
}
