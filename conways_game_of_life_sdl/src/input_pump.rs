use crate::IResult;
use sdl2::{event::Event, keyboard::Keycode, mouse::MouseButton, EventPump, Sdl};

struct Mouse {
    left: bool,
    right: bool,
}

impl Mouse {
    fn new() -> Self {
        Self {
            left: false,
            right: false,
        }
    }
}

struct InIter {
    mouse: Mouse,
    zoom_scaling: f64,
}

pub struct InputPump {
    event_pump: EventPump,
    ii: InIter,
}

pub struct InputIterator<'a> {
    poll_iter: sdl2::event::EventPollIterator<'a>,
    ii: &'a mut InIter,
}

pub enum Input {
    MoveCamera { x: i32, y: i32 },
    ZoomCamera { zoom: f64 },
    Pause,
    Run,
    Reset,
    Quit,
}

impl Iterator for InputIterator<'_> {
    type Item = Input;
    fn next(&mut self) -> Option<Self::Item> {
        fn match_mouse(mouse: &mut Mouse, mouse_btn: MouseButton, set: bool) {
            match mouse_btn {
                MouseButton::Left => mouse.left = set,
                MouseButton::Right => mouse.right = set,
                _ => (),
            }
        }
        Some(match self.poll_iter.next()? {
            Event::Quit { .. }
            | Event::KeyUp {
                keycode: Some(Keycode::Escape),
                ..
            } => Input::Quit,
            Event::KeyUp {
                keycode: Some(key), ..
            } => match key {
                Keycode::R => Input::Reset,
                Keycode::Space => Input::Pause,
                _ => Input::Run,
            },
            Event::MouseButtonDown { mouse_btn, .. } => {
                match_mouse(&mut self.ii.mouse, mouse_btn, true);
                Input::Run
            }
            Event::MouseButtonUp { mouse_btn, .. } => {
                match_mouse(&mut self.ii.mouse, mouse_btn, false);
                Input::Run
            }
            Event::MouseMotion { xrel, yrel, .. } => {
                if self.ii.mouse.left {
                    Input::MoveCamera { x: -xrel, y: -yrel }
                } else {
                    Input::Run
                }
            }
            Event::MouseWheel { y, .. } => Input::ZoomCamera {
                zoom: y as f64 * self.ii.zoom_scaling,
            },
            _ => Input::Run,
        })
    }
}

impl InputPump {
    pub fn new(sdl: &Sdl) -> IResult<Self> {
        Ok(Self {
            event_pump: sdl.event_pump()?,
            ii: InIter {
                mouse: Mouse::new(),
                zoom_scaling: 0.1,
            },
        })
    }

    pub fn poll_iter(&mut self) -> impl Iterator<Item = Input> + '_ {
        InputIterator {
            poll_iter: self.event_pump.poll_iter(),
            ii: &mut self.ii,
        }
    }
}
