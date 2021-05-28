use crate::{Grid, GridPoint};
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

pub trait ColorModulator {
    fn color(&self) -> Color;
    fn reset(&mut self);
    fn modulate(&mut self);
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
}

impl ColorModulator for CyclicalModulator {
    fn color(&self) -> Color {
        self.color_state
    }

    fn reset(&mut self) {
        self.color_state = self.opt.into();
    }

    fn modulate(&mut self) {
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

pub trait NewCellColor {
    fn update<G: Grid>(&mut self, grid: &G);
    fn get_cell_color(&mut self, point: GridPoint, cell: bool) -> Color;
    fn reset(&mut self);
}

pub struct NewCellColorCyclical {
    pub cyclical_modulator: CyclicalModulator,
    cell_states: Vec<Vec<(bool, Color)>>,
}

impl NewCellColorCyclical {
    pub fn new(cyclical_modulator: CyclicalModulator) -> Self {
        Self {
            cyclical_modulator,
            cell_states: Vec::new(),
        }
    }
}

impl NewCellColor for NewCellColorCyclical {
    fn update<G: Grid>(&mut self, grid: &G) {
        self.cyclical_modulator.modulate();
        if self.cell_states.is_empty() {
            let size = grid.size();
            self.cell_states = vec![
                vec![(false, self.cyclical_modulator.color()); size.0 as usize];
                size.1 as usize
            ];
            grid.inspect(|(x, y), grid| {
                self.cell_states[y as usize][x as usize].0 = grid.get_cell_unchecked((x, y));
            });
        }
    }

    fn get_cell_color(&mut self, (x, y): GridPoint, cell: bool) -> Color {
        let cell_state = &mut self.cell_states[y as usize][x as usize];
        if cell {
            if !cell_state.0 {
                cell_state.1 = self.cyclical_modulator.color();
                cell_state.0 = true;
            }
        } else {
            cell_state.0 = false;
        }
        cell_state.1
    }

    fn reset(&mut self) {
        self.cyclical_modulator.reset();
    }
}

pub struct NewCellColorHeatMap {
    cell_states: Vec<Vec<(bool, Color)>>,
    hot: Rgb,
    cold: Rgb,
}

impl NewCellColorHeatMap {
    pub fn new(hot: Rgb, cold: Rgb) -> Self {
        Self {
            cell_states: Vec::new(),
            hot,
            cold,
        }
    }
}

impl NewCellColor for NewCellColorHeatMap {
    fn update<G: Grid>(&mut self, grid: &G) {
        if self.cell_states.is_empty() {
            let size = grid.size();
            self.cell_states =
                vec![vec![(false, self.hot.into()); size.0 as usize]; size.1 as usize];
        }
    }

    fn get_cell_color(&mut self, (x, y): GridPoint, cell: bool) -> Color {
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

    fn reset(&mut self) {
        let hot = self.hot.into();
        for row in &mut self.cell_states {
            for cell_state in row {
                *cell_state = (false, hot);
            }
        }
    }
}
