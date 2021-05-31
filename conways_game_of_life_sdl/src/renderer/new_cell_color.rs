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

struct CellStates {
    pub cells: Vec<Vec<CellState>>,
}

impl CellStates {
    fn new(color: Color, grid_size: GridPoint) -> Self {
        Self {
            cells: vec![
                vec![
                    CellState {
                        state: false,
                        color
                    };
                    grid_size.0 as usize
                ];
                grid_size.1 as usize
            ],
        }
    }

    fn get_cell(&mut self, (x, y): GridPoint) -> &mut CellState {
        &mut self.cells[y as usize][x as usize]
    }

    fn reset(&mut self, color: Color) {
        for row in &mut self.cells {
            for cell_state in row {
                *cell_state = CellState {
                    state: false,
                    color,
                };
            }
        }
    }
}

pub struct NewCellColorCyclical {
    pub cyclical_modulator: CyclicalModulator,
    cell_states: CellStates,
}

impl NewCellColorCyclical {
    pub fn new(cyclical_modulator: CyclicalModulator, grid_size: GridPoint) -> Self {
        Self {
            cell_states: CellStates::new(cyclical_modulator.color(), grid_size),
            cyclical_modulator,
        }
    }

    pub fn get_cell_color(&mut self, point: GridPoint, cell: bool) -> Color {
        let cell_state = self.cell_states.get_cell(point);
        if cell {
            if !cell_state.state {
                cell_state.color = self.cyclical_modulator.color();
                cell_state.state = true;
            }
        } else {
            cell_state.state = false;
        }
        cell_state.color
    }

    pub fn update(&mut self) {
        self.cyclical_modulator.modulate();
    }

    pub fn reset(&mut self) {
        self.cyclical_modulator.reset();
        self.cell_states.reset(self.cyclical_modulator.color());
    }
}

pub struct NewCellColorHeatMap {
    cell_states: CellStates,
    hot: Rgb,
    cold: Rgb,
}

impl NewCellColorHeatMap {
    pub fn new(hot: Rgb, cold: Rgb, grid_size: GridPoint) -> Self {
        Self {
            cell_states: CellStates::new(hot.into(), grid_size),
            hot,
            cold,
        }
    }

    pub fn get_cell_color(&mut self, point: GridPoint, cell: bool) -> Color {
        let cell_state = self.cell_states.get_cell(point);
        if cell {
            if !cell_state.state {
                cell_state.color = self.hot.into();
                cell_state.state = true;
            }
        } else {
            cell_state.state = false;
        }
        cell_state.color
    }

    pub fn update(&mut self) {
        for row in &mut self.cell_states.cells {
            for cell_state in row {
                match self.hot {
                    Rgb::Red => {
                        if cell_state.color.r > 0 {
                            cell_state.color.r -= 1;
                        }
                    }
                    Rgb::Green => {
                        if cell_state.color.g > 0 {
                            cell_state.color.g -= 1;
                        }
                    }
                    Rgb::Blue => {
                        if cell_state.color.b > 0 {
                            cell_state.color.b -= 1;
                        }
                    }
                }
                match self.cold {
                    Rgb::Red => {
                        if cell_state.color.r < u8::MAX {
                            cell_state.color.r += 1;
                        }
                    }
                    Rgb::Green => {
                        if cell_state.color.g < u8::MAX {
                            cell_state.color.g += 1;
                        }
                    }
                    Rgb::Blue => {
                        if cell_state.color.b < u8::MAX {
                            cell_state.color.b += 1;
                        }
                    }
                }
            }
        }
    }

    pub fn reset(&mut self) {
        self.cell_states.reset(self.hot.into());
    }
}
