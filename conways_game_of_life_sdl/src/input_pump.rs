use crate::{BResult, Grid, GridPoint, IResult};
use sdl2::{
    event::Event,
    keyboard::Keycode,
    mouse::{MouseButton, MouseState, MouseUtil},
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
    pub draw_state: Option<(GridPoint, bool)>,
    event_pump: EventPump,
    mouse: Mouse,
    mouse_util: MouseUtil,
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
            draw_state: None,
            mouse: event_pump.mouse_state().into(),
            mouse_util: sdl.mouse(),
            event_pump,
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
                    Input::DrawCell {
                        point: (self.mouse.x, self.mouse.y),
                    }
                }
                MouseButton::Right => {
                    self.mouse.right = true;
                    Input::Run
                }
                _ => Input::Run,
            },
            Event::MouseButtonUp { mouse_btn, .. } => {
                match mouse_btn {
                    MouseButton::Left => {
                        self.mouse.left = false;
                        self.draw_state = None;
                    }
                    MouseButton::Right => {
                        self.mouse.right = false;
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
                if self.mouse.right {
                    Input::MoveCamera { x: -xrel, y: -yrel }
                } else if self.mouse.left {
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

    pub fn mouse_in_window(&self) -> bool {
        self.mouse_util.focused_window_id().is_some()
    }

    pub fn draw<G: Grid>(&mut self, grid: &mut G, point: GridPoint) -> BResult<()> {
        let ret = if let Some((ref mut prev_point, prev_state)) = self.draw_state {
            if *prev_point != point {
                let ret = grid.set_line(*prev_point, point, prev_state);
                *prev_point = point;
                ret
            } else {
                Ok(())
            }
        } else {
            let state = !grid.get_cell_unchecked(point);
            self.draw_state = Some((point, state));
            grid.set_cell(point, state)
        };
        if !self.mouse_in_window() {
            self.draw_state = None;
        }
        ret
    }
}
