pub mod config;
mod frame_regulator;

use frame_regulator::FrameRegulator;
use std::{error::Error, fmt, mem};

pub type GridUnit = u32;
pub type GridPoint = (GridUnit, GridUnit);

fn grid_point_contained(point: GridPoint, size: GridPoint) -> bool {
    point.0 < size.0 && point.1 < size.1
}

#[derive(Debug)]
pub struct OutOfBounds {
    point: GridPoint,
    size: GridPoint,
}
impl fmt::Display for OutOfBounds {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl Error for OutOfBounds {}

pub type BResult<T> = Result<T, OutOfBounds>;

pub enum SetLineOpt {
    Horizontal,
    Vertical,
}

pub trait Grid {
    fn empty(size: GridPoint) -> Self
    where
        Self: Sized;
    fn size(&self) -> GridPoint;
    fn set_cell(&mut self, point: GridPoint, b: bool) -> BResult<()>;
    fn set_fps(&mut self, fps: u64);
    fn update(&mut self);

    fn get_cell_unchecked(&self, point: GridPoint) -> bool;
    fn get_cell_unchecked_mut(&mut self, point: GridPoint) -> &mut bool;

    fn get_cell(&self, point: GridPoint) -> bool {
        if grid_point_contained(point, self.size()) {
            self.get_cell_unchecked(point)
        } else {
            false
        }
    }

    fn get_cell_mut(&mut self, point: GridPoint) -> BResult<&mut bool> {
        let size = self.size();
        if grid_point_contained(point, size) {
            Ok(self.get_cell_unchecked_mut(point))
        } else {
            Err(OutOfBounds { point, size })
        }
    }

    fn inspect<F: FnMut(GridPoint, &Self)>(&self, mut f: F) {
        let size = self.size();
        for y in 0..size.1 {
            for x in 0..size.0 {
                f((x, y), self);
            }
        }
    }

    fn try_inspect<E, F: FnMut(GridPoint, &Self) -> Result<(), E>>(
        &self,
        mut f: F,
    ) -> Result<(), E> {
        let size = self.size();
        for y in 0..size.1 {
            for x in 0..size.0 {
		f((x, y), self)?;
            }
        }
        Ok(())
    }

    fn inspect_mut<F: FnMut(GridPoint, &mut Self)>(&mut self, mut f: F) {
        let size = self.size();
        for y in 0..size.1 {
            for x in 0..size.0 {
		f((x, y), self);
	    }
	}
    }

    fn try_inspect_mut<E, F: FnMut(GridPoint, &mut Self) -> Result<(), E>>(&mut self, mut f: F) -> Result<(), E> {
        let size = self.size();
        for y in 0..size.1 {
            for x in 0..size.0 {
		f((x, y), self)?;
	    }
	}
	Ok(())
    }
    
    fn g_fmt(&self, _tc: char, _fc: char, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Ok(())
    }

    fn next_cell_state(&self, (x, y): GridPoint) -> bool {
        let mut counter = 0;

        if x != 0 {
            if self.get_cell((x - 1, y)) {
                counter += 1;
            }
            if self.get_cell((x - 1, y + 1)) {
                counter += 1;
            }
        }
        if y != 0 {
            if self.get_cell((x, y - 1)) {
                counter += 1;
            }
            if self.get_cell((x + 1, y - 1)) {
                counter += 1;
            }
        }
        if x != 0 && y != 0 && self.get_cell((x - 1, y - 1)) {
            counter += 1;
        }

        if self.get_cell((x, y + 1)) {
            counter += 1;
        }
        if self.get_cell((x + 1, y)) {
            counter += 1;
        }
        if self.get_cell((x + 1, y + 1)) {
            counter += 1;
        }

        counter == 3 || (counter == 2 && self.get_cell_unchecked((x, y)))
    }

    fn set_hline(
        &mut self,
        start: GridUnit,
        length: GridUnit,
        y: GridUnit,
        b: bool,
    ) -> BResult<()> {
        self.set_line(start, length, y, b, SetLineOpt::Horizontal)
    }

    fn set_vline(
        &mut self,
        start: GridUnit,
        length: GridUnit,
        x: GridUnit,
        b: bool,
    ) -> BResult<()> {
        self.set_line(start, length, x, b, SetLineOpt::Vertical)
    }

    fn set_line(
        &mut self,
        start: GridUnit,
        length: GridUnit,
        other: GridUnit,
        b: bool,
        opt: SetLineOpt,
    ) -> BResult<()> {
        if length == 1 {
            self.set_cell((start, other), b)?;
        } else {
            for i in start..start + length {
                let point = match opt {
                    SetLineOpt::Horizontal => (i, other),
                    SetLineOpt::Vertical => (other, i),
                };
                self.set_cell(point, b)?;
            }
        }
        Ok(())
    }
}

pub struct LinearGrid {
    size: GridPoint,
    current_vec: Vec<bool>,
    next_vec: Vec<bool>,
    frame_regulator_opt: Option<FrameRegulator>,
}

impl LinearGrid {
    pub fn empty(size: GridPoint) -> Self {
        let next_vec = vec![false; (size.0 * size.1) as usize];
        Self {
            size,
            current_vec: next_vec.clone(),
            next_vec,
            frame_regulator_opt: Some(FrameRegulator::fps(10)),
        }
    }

    pub fn as_slice(&self) -> &[bool] {
        &self.current_vec
    }

    fn get_index(&self, point: GridPoint) -> usize {
        (point.0 + point.1 * self.size.0) as usize
    }
}

impl Grid for LinearGrid {
    fn empty(size: GridPoint) -> Self {
        Self::empty(size)
    }

    fn size(&self) -> GridPoint {
        self.size
    }

    fn set_cell(&mut self, point: GridPoint, b: bool) -> BResult<()> {
        *self.get_cell_mut(point)? = b;
        Ok(())
    }

    fn set_fps(&mut self, fps: u64) {
        self.frame_regulator_opt = if fps != 0 {
            Some(FrameRegulator::fps(fps))
        } else {
            None
        }
    }

    fn update(&mut self) {
        for x in 0..self.size.0 {
            for y in 0..self.size.1 {
                let index = self.get_index((x, y));
                self.next_vec[index] = self.next_cell_state((x, y));
            }
        }
        mem::swap(&mut self.current_vec, &mut self.next_vec);

        if let Some(frame_regulator) = &mut self.frame_regulator_opt {
            frame_regulator.regulate();
        }
    }

    fn get_cell_unchecked(&self, point: GridPoint) -> bool {
        self.current_vec[self.get_index(point)]
    }

    fn get_cell_unchecked_mut(&mut self, point: GridPoint) -> &mut bool {
        let index = self.get_index(point);
        &mut self.current_vec[index]
    }

    fn g_fmt(&self, tc: char, fc: char, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, b) in self.as_slice().iter().enumerate() {
            write!(f, "{}", if *b { tc } else { fc })?;
            if (i + 1) % self.size.0 as usize == 0 {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

impl fmt::Display for LinearGrid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.g_fmt('\u{2588}', ' ', f)
    }
}

impl fmt::Debug for LinearGrid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.g_fmt('X', '-', f)
    }
}

pub struct Grid2d {
    size: GridPoint,
    current_vec: Vec<Vec<bool>>,
    next_vec: Vec<Vec<bool>>,
    frame_regulator_opt: Option<FrameRegulator>,
}

impl Grid2d {
    pub fn empty(size: GridPoint) -> Self {
        let next_vec = vec![vec![false; size.0 as usize]; size.1 as usize];
        Self {
            size,
            current_vec: next_vec.clone(),
            next_vec,
            frame_regulator_opt: Some(FrameRegulator::fps(10)),
        }
    }
}

impl Grid for Grid2d {
    fn empty(size: GridPoint) -> Self {
        Self::empty(size)
    }

    fn size(&self) -> GridPoint {
        self.size
    }

    fn set_cell(&mut self, point: GridPoint, b: bool) -> BResult<()> {
        *self.get_cell_mut(point)? = b;
        Ok(())
    }

    fn set_fps(&mut self, fps: u64) {
        self.frame_regulator_opt = if fps != 0 {
            Some(FrameRegulator::fps(fps))
        } else {
            None
        }
    }

    fn update(&mut self) {
        for x in 0..self.size.0 {
            for y in 0..self.size.1 {
                self.next_vec[y as usize][x as usize] = self.next_cell_state((x, y));
            }
        }
        mem::swap(&mut self.current_vec, &mut self.next_vec);

        if let Some(frame_regulator) = &mut self.frame_regulator_opt {
            frame_regulator.regulate();
        }
    }

    fn get_cell_unchecked(&self, (x, y): GridPoint) -> bool {
        self.current_vec[y as usize][x as usize]
    }

    fn get_cell_unchecked_mut(&mut self, (x, y): GridPoint) -> &mut bool {
        &mut self.current_vec[y as usize][x as usize]
    }

    fn g_fmt(&self, tc: char, fc: char, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for v in &self.current_vec {
            for b in v {
                write!(f, "{}", if *b { tc } else { fc })?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl fmt::Display for Grid2d {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.g_fmt('\u{2588}', ' ', f)
    }
}

impl fmt::Debug for Grid2d {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.g_fmt('X', '-', f)
    }
}
