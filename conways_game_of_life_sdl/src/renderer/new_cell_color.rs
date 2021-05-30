use crate::GridPoint;
use sdl2::pixels::Color;

#[derive(Clone, Copy)]
pub enum Rgb {
    Red,
    Green,
    Blue,
}

impl From<Rgb> for Color {
    fn from(rgb: Rgb) -> Self {
        match rgb {
            Rgb::Red => Self::RGB(255, 0, 0),
            Rgb::Green => Self::RGB(0, 255, 0),
            Rgb::Blue => Self::RGB(0, 0, 255),
        }
    }
}

#[derive(Clone, Copy)]
pub enum Rygcbm {
    Red,
    Yellow,
    Green,
    Cyan,
    Blue,
    Magenta,
}

impl From<Rygcbm> for Color {
    fn from(rygcbm: Rygcbm) -> Self {
        match rygcbm {
            Rygcbm::Red => Self::RGB(255, 0, 0),
            Rygcbm::Yellow => Self::RGB(255, 255, 0),
            Rygcbm::Green => Self::RGB(0, 255, 0),
            Rygcbm::Cyan => Self::RGB(0, 255, 255),
            Rygcbm::Blue => Self::RGB(0, 0, 255),
            Rygcbm::Magenta => Self::RGB(255, 0, 255),
        }
    }
}

#[derive(Clone, Copy)]
pub enum CyclicalModulatorOpt {
    Rgb(Rgb),
    Rygcbm(Rygcbm),
}

impl From<CyclicalModulatorOpt> for Color {
    fn from(cmo: CyclicalModulatorOpt) -> Color {
        match cmo {
            CyclicalModulatorOpt::Rgb(rgb) => rgb.into(),
            CyclicalModulatorOpt::Rygcbm(rygcbm) => rygcbm.into(),
        }
    }
}

pub struct CyclicalModulator {
    color_state: Color,
    opt: CyclicalModulatorOpt,
}

impl CyclicalModulator {
    pub fn new(opt: CyclicalModulatorOpt) -> Self {
        Self {
            color_state: opt.into(),
            opt,
        }
    }

    pub fn color(&self) -> Color {
        self.color_state
    }

    pub fn reset(&mut self) {
        self.color_state = self.opt.into();
    }

    pub fn modulate(&mut self) {
        match self.opt {
            CyclicalModulatorOpt::Rygcbm(_) => {
                if self.color_state.r == u8::MAX {
                    if self.color_state.b > 0 {
                        self.color_state.b -= 1;
                    } else if self.color_state.g < u8::MAX {
                        self.color_state.g += 1;
                    }
                }
                if self.color_state.g == u8::MAX {
                    if self.color_state.r > 0 {
                        self.color_state.r -= 1;
                    } else if self.color_state.b < u8::MAX {
                        self.color_state.b += 1;
                    }
                }
                if self.color_state.b == u8::MAX {
                    if self.color_state.g > 0 {
                        self.color_state.g -= 1;
                    } else if self.color_state.r < u8::MAX {
                        self.color_state.r += 1;
                    }
                }
            }
            CyclicalModulatorOpt::Rgb(_) => {
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
    }
}

#[derive(Clone, Copy)]
struct CellState {
    state: bool,
    color: Color,
}

impl CellState {
    fn get_cell_color<F: FnOnce() -> Color>(&mut self, cell: bool, color_fn: F) -> Color {
        if cell {
            if !self.state {
                self.color = color_fn();
                self.state = true;
            }
        } else {
            self.state = false;
        }
        self.color
    }
}

pub struct NewCellColorCyclical {
    pub cyclical_modulator: CyclicalModulator,
    cell_states: Vec<Vec<CellState>>,
}

impl NewCellColorCyclical {
    pub fn new(cyclical_modulator: CyclicalModulator, grid_size: GridPoint) -> Self {
        Self {
            cell_states: vec![
                vec![CellState { state: false, color: cyclical_modulator.color() }; grid_size.0 as usize];
                grid_size.1 as usize
            ],
            cyclical_modulator,
        }
    }

    pub fn get_cell_color(&mut self, (x, y): GridPoint, cell: bool) -> Color {
	let cyclical_modulator = &self.cyclical_modulator;
        self.cell_states[y as usize][x as usize].get_cell_color(cell, move || {cyclical_modulator.color()})
    }

    pub fn reset(&mut self) {
        self.cyclical_modulator.reset();
        let color = self.cyclical_modulator.color();
        for row in &mut self.cell_states {
            for cell_state in row {
                *cell_state = CellState{ state: false, color };
            }
        }
    }
}

pub struct NewCellColorHeatMap {
    cell_states: Vec<Vec<(bool, Color)>>,
    hot: Rgb,
    cold: Rgb,
}

impl NewCellColorHeatMap {
    pub fn new(hot: Rgb, cold: Rgb, grid_size: GridPoint) -> Self {
        Self {
            cell_states: vec![
                vec![(false, hot.into()); grid_size.0 as usize];
                grid_size.1 as usize
            ],
            hot,
            cold,
        }
    }

    pub fn get_cell_color(&mut self, (x, y): GridPoint, cell: bool) -> Color {
        let cell_state = &mut self.cell_states[y as usize][x as usize];
        if cell {
            if !cell_state.0 {
                cell_state.1 = self.hot.into();
                cell_state.0 = true;
            } else {
                match self.hot {
                    Rgb::Red => {
                        if cell_state.1.r > 0 {
                            cell_state.1.r -= 1;
                        }
                    }
                    Rgb::Green => {
                        if cell_state.1.g > 0 {
                            cell_state.1.g -= 1;
                        }
                    }
                    Rgb::Blue => {
                        if cell_state.1.b > 0 {
                            cell_state.1.b -= 1;
                        }
                    }
                }
                match self.cold {
                    Rgb::Red => {
                        if cell_state.1.r < u8::MAX {
                            cell_state.1.r += 1;
                        }
                    }
                    Rgb::Green => {
                        if cell_state.1.g < u8::MAX {
                            cell_state.1.g += 1;
                        }
                    }
                    Rgb::Blue => {
                        if cell_state.1.b < u8::MAX {
                            cell_state.1.b += 1;
                        }
                    }
                }
            }
        } else {
            cell_state.0 = false;
        }
        cell_state.1
    }

    pub fn reset(&mut self) {
        let hot = self.hot.into();
        for row in &mut self.cell_states {
            for cell_state in row {
                *cell_state = (false, hot);
            }
        }
    }
}
