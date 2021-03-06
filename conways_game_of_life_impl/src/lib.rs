pub mod config;
mod frame_regulator;

use frame_regulator::FrameRegulator;
pub use frame_regulator::ZeroFps;
use std::{
    alloc::{self, Layout},
    error::Error,
    fmt, mem,
};

pub type GridUnit = u16;
pub type GridPoint = (GridUnit, GridUnit);

fn grid_point_contained(point: GridPoint, size: GridPoint) -> bool {
    point.0 < size.0 && point.1 < size.1
}

#[derive(Debug)]
pub struct OutOfBounds {
    point: GridPoint,
    size: GridPoint,
}
impl OutOfBounds {
    pub fn point(&self) -> GridPoint {
        self.point
    }
    pub fn size(&self) -> GridPoint {
        self.size
    }
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

mod private {
    use super::*;
    pub trait GridPrivate {
        fn _size(&self) -> GridPoint;
        fn _frame_regulator_opt(&mut self) -> &mut Option<FrameRegulator>;
        fn _get_next_cell_unchecked_mut(&mut self, point: GridPoint) -> &mut bool;

        fn _regulate_frame(&mut self) {
            if let Some(frame_regulator) = &mut self._frame_regulator_opt() {
                frame_regulator.regulate();
            }
        }

        fn _g_fmt(&self, _tc: char, _fc: char, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
            Ok(())
        }
    }
}
use private::GridPrivate;

pub trait Grid: GridPrivate {
    fn update(&mut self);

    fn get_cell_unchecked(&self, point: GridPoint) -> bool;
    fn get_cell_unchecked_mut(&mut self, point: GridPoint) -> &mut bool;

    fn size(&self) -> GridPoint {
        self._size()
    }

    fn set_fps(&mut self, fps: u64) {
        *self._frame_regulator_opt() = if fps != 0 {
            Some(FrameRegulator::fps(fps).unwrap())
        } else {
            None
        };
    }

    fn set_cell(&mut self, point: GridPoint, b: bool) -> BResult<()> {
        *self.get_cell_mut(point)? = b;
        Ok(())
    }

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

    fn try_inspect_mut<E, F: FnMut(GridPoint, &mut Self) -> Result<(), E>>(
        &mut self,
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

    fn set_hline(
        &mut self,
        start: GridUnit,
        length: GridUnit,
        y: GridUnit,
        b: bool,
    ) -> BResult<()> {
        self.set_aaline(start, length, y, b, SetLineOpt::Horizontal)
    }

    fn set_vline(
        &mut self,
        start: GridUnit,
        length: GridUnit,
        x: GridUnit,
        b: bool,
    ) -> BResult<()> {
        self.set_aaline(start, length, x, b, SetLineOpt::Vertical)
    }

    fn set_aaline(
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

    fn set_line(&mut self, mut start: GridPoint, mut end: GridPoint, b: bool) -> BResult<()> {
        let axis = if (end.1 as i32 - start.1 as i32).abs() > (end.0 as i32 - start.0 as i32).abs()
        {
            mem::swap(&mut start.0, &mut start.1);
            mem::swap(&mut end.0, &mut end.1);
            if end.0 < start.0 {
                mem::swap(&mut start, &mut end);
            }
            false
        } else {
            if end.0 < start.0 {
                mem::swap(&mut start, &mut end);
            }
            true
        };

        let mut set_cell = |v, o_v, x_axis| -> BResult<()> {
            self.set_cell(if x_axis { (v, o_v) } else { (o_v, v) }, b)
        };

        if end.0 - start.0 == 0 {
            set_cell(start.0, start.1, axis)?;
        } else {
            let start_x = start.0 as f64;
            let start_y = start.1 as f64;
            let m = (end.1 as f64 - start_y) / (end.0 as f64 - start_x);
            let c = start_y - m * start_x;
            for v in start.0..=end.0 {
                set_cell(v, (m * v as f64 + c).round() as GridUnit, axis)?;
            }
        }
        Ok(())
    }
}

fn next_cell_state_scan_wrap_around<G: Grid>(grid: &G, (x, y): GridPoint) -> bool {
    let size = grid.size();

    fn get_range(v: GridUnit, size: GridUnit) -> [GridUnit; 3] {
        let v = v as i32;
        let range = size as i32 - 1;
        [
            if v == 0 { range } else { v - 1 } as GridUnit,
            v as GridUnit,
            if v == range { 0 } else { v + 1 } as GridUnit,
        ]
    }
    let x_range = get_range(x, size.0);
    let y_range = get_range(y, size.1);
    let mut counter = 0;

    for &y_scan in &y_range {
        for &x_scan in &x_range {
            if (x_scan != x || y_scan != y) && grid.get_cell_unchecked((x_scan, y_scan)) {
                counter += 1;
            }
        }
    }

    counter == 3 || (counter == 2 && grid.get_cell_unchecked((x, y)))
}

#[allow(dead_code)]
fn next_cell_state_individual<G: Grid>(grid: &G, (x, y): GridPoint) -> bool {
    let mut counter = 0;

    if x > 0 {
        if grid.get_cell((x - 1, y)) {
            counter += 1;
        }
        if grid.get_cell((x - 1, y + 1)) {
            counter += 1;
        }
    }
    if y > 0 {
        if grid.get_cell((x, y - 1)) {
            counter += 1;
        }
        if grid.get_cell((x + 1, y - 1)) {
            counter += 1;
        }
    }
    if x > 0 && y > 0 && grid.get_cell((x - 1, y - 1)) {
        counter += 1;
    }

    if grid.get_cell((x, y + 1)) {
        counter += 1;
    }
    if grid.get_cell((x + 1, y)) {
        counter += 1;
    }
    if grid.get_cell((x + 1, y + 1)) {
        counter += 1;
    }

    counter == 3 || (counter == 2 && grid.get_cell_unchecked((x, y)))
}

fn set_next_state<G: Grid>(grid: &mut G) {
    let size = grid.size();
    let x_max = size.0 - 1;
    let y_max = size.1 - 1;

    for y in 1..y_max {
        for x in 1..x_max {
            let mut counter = 0;

            if grid.get_cell_unchecked((x - 1, y - 1)) {
                counter += 1;
            }
            if grid.get_cell_unchecked((x, y - 1)) {
                counter += 1;
            }
            if grid.get_cell_unchecked((x + 1, y - 1)) {
                counter += 1;
            }

            if grid.get_cell_unchecked((x - 1, y)) {
                counter += 1;
            }
            if grid.get_cell_unchecked((x + 1, y)) {
                counter += 1;
            }

            if grid.get_cell_unchecked((x - 1, y + 1)) {
                counter += 1;
            }
            if grid.get_cell_unchecked((x, y + 1)) {
                counter += 1;
            }
            if grid.get_cell_unchecked((x + 1, y + 1)) {
                counter += 1;
            }
            *grid._get_next_cell_unchecked_mut((x, y)) =
                counter == 3 || (counter == 2 && grid.get_cell_unchecked((x, y)));
        }
    }

    for x in 0..size.0 {
        *grid._get_next_cell_unchecked_mut((x, 0)) = next_cell_state_scan_wrap_around(grid, (x, 0));
        *grid._get_next_cell_unchecked_mut((x, y_max)) =
            next_cell_state_scan_wrap_around(grid, (x, y_max));
    }
    for y in 1..y_max {
        *grid._get_next_cell_unchecked_mut((0, y)) = next_cell_state_scan_wrap_around(grid, (0, y));
        *grid._get_next_cell_unchecked_mut((x_max, y)) =
            next_cell_state_scan_wrap_around(grid, (x_max, y));
    }
}

type Grid1dVecContainer = Vec<bool>;

#[derive(Clone)]
pub struct Grid1dVec {
    size: GridPoint,
    current_vec: Grid1dVecContainer,
    next_vec: Grid1dVecContainer,
    frame_regulator_opt: Option<FrameRegulator>,
}

impl Grid1dVec {
    pub fn empty(size: GridPoint) -> Self {
        let next_vec = vec![false; size.0 as usize * size.1 as usize];
        Self {
            size,
            current_vec: next_vec.clone(),
            next_vec,
            frame_regulator_opt: None,
        }
    }

    pub fn as_slice(&self) -> &[bool] {
        &self.current_vec
    }

    fn get_index(&self, point: GridPoint) -> usize {
        point.0 as usize + point.1 as usize * self.size.0 as usize
    }
}

impl GridPrivate for Grid1dVec {
    fn _size(&self) -> GridPoint {
        self.size
    }

    fn _frame_regulator_opt(&mut self) -> &mut Option<FrameRegulator> {
        &mut self.frame_regulator_opt
    }

    fn _get_next_cell_unchecked_mut(&mut self, point: GridPoint) -> &mut bool {
        let index = self.get_index(point);
        &mut self.next_vec[index]
    }

    fn _g_fmt(&self, tc: char, fc: char, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, b) in self.as_slice().iter().enumerate() {
            write!(f, "{}", if *b { tc } else { fc })?;
            if (i + 1) % self.size.0 as usize == 0 {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

impl Grid for Grid1dVec {
    fn update(&mut self) {
        set_next_state(self);
        mem::swap(&mut self.current_vec, &mut self.next_vec);
        self._regulate_frame();
    }

    fn get_cell_unchecked(&self, point: GridPoint) -> bool {
        self.current_vec[self.get_index(point)]
    }

    fn get_cell_unchecked_mut(&mut self, point: GridPoint) -> &mut bool {
        let index = self.get_index(point);
        &mut self.current_vec[index]
    }
}

impl fmt::Display for Grid1dVec {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self._g_fmt('\u{2588}', ' ', f)
    }
}

impl fmt::Debug for Grid1dVec {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self._g_fmt('X', '-', f)
    }
}

type Grid2dVecContainer = Vec<Vec<bool>>;

#[derive(Clone)]
pub struct Grid2dVec {
    size: GridPoint,
    current_vec: Grid2dVecContainer,
    next_vec: Grid2dVecContainer,
    frame_regulator_opt: Option<FrameRegulator>,
}

impl Grid2dVec {
    pub fn empty(size: GridPoint) -> Self {
        let next_vec = vec![vec![false; size.0 as usize]; size.1 as usize];
        Self {
            size,
            current_vec: next_vec.clone(),
            next_vec,
            frame_regulator_opt: None,
        }
    }
}

impl GridPrivate for Grid2dVec {
    fn _size(&self) -> GridPoint {
        self.size
    }

    fn _frame_regulator_opt(&mut self) -> &mut Option<FrameRegulator> {
        &mut self.frame_regulator_opt
    }

    fn _get_next_cell_unchecked_mut(&mut self, (x, y): GridPoint) -> &mut bool {
        &mut self.next_vec[y as usize][x as usize]
    }

    fn _g_fmt(&self, tc: char, fc: char, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.current_vec {
            for b in row {
                write!(f, "{}", if *b { tc } else { fc })?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Grid for Grid2dVec {
    fn update(&mut self) {
        set_next_state(self);
        mem::swap(&mut self.current_vec, &mut self.next_vec);
        self._regulate_frame();
    }

    fn get_cell_unchecked(&self, (x, y): GridPoint) -> bool {
        self.current_vec[y as usize][x as usize]
    }

    fn get_cell_unchecked_mut(&mut self, (x, y): GridPoint) -> &mut bool {
        &mut self.current_vec[y as usize][x as usize]
    }
}

impl fmt::Display for Grid2dVec {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self._g_fmt('\u{2588}', ' ', f)
    }
}

impl fmt::Debug for Grid2dVec {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self._g_fmt('X', '-', f)
    }
}

type Grid2dArrContainer<const WIDTH: usize, const HEIGHT: usize> = [[bool; WIDTH]; HEIGHT];

pub struct Grid2dArr<const WIDTH: usize, const HEIGHT: usize> {
    current_arr: Grid2dArrContainer<WIDTH, HEIGHT>,
    next_arr: Grid2dArrContainer<WIDTH, HEIGHT>,
    frame_regulator_opt: Option<FrameRegulator>,
}

impl<const WIDTH: usize, const HEIGHT: usize> Grid2dArr<WIDTH, HEIGHT> {
    /*Only allow creation of boxed Grid2dArr to prevent possible stack overflow. Can only
    allocate directly on the heap using unsafe code (Box::new still causes stack overflow).*/
    pub fn empty() -> Box<Self> {
        let mut ret = unsafe { Box::from_raw(alloc::alloc(Layout::new::<Self>()) as *mut Self) };
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                ret.current_arr[y][x] = false;
                ret.next_arr[y][x] = false;
            }
        }
        ret.frame_regulator_opt = None;
        ret
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> private::GridPrivate for Grid2dArr<WIDTH, HEIGHT> {
    fn _size(&self) -> GridPoint {
        (WIDTH as GridUnit, HEIGHT as GridUnit)
    }

    fn _frame_regulator_opt(&mut self) -> &mut Option<FrameRegulator> {
        &mut self.frame_regulator_opt
    }

    fn _get_next_cell_unchecked_mut(&mut self, (x, y): GridPoint) -> &mut bool {
        &mut self.next_arr[y as usize][x as usize]
    }

    fn _g_fmt(&self, tc: char, fc: char, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.current_arr {
            for b in row {
                write!(f, "{}", if *b { tc } else { fc })?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> Grid for Grid2dArr<WIDTH, HEIGHT> {
    fn update(&mut self) {
        set_next_state(self);
        self.current_arr = self.next_arr;
        self._regulate_frame();
    }

    fn get_cell_unchecked(&self, (x, y): GridPoint) -> bool {
        self.current_arr[y as usize][x as usize]
    }

    fn get_cell_unchecked_mut(&mut self, (x, y): GridPoint) -> &mut bool {
        &mut self.current_arr[y as usize][x as usize]
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> fmt::Display for Grid2dArr<WIDTH, HEIGHT> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self._g_fmt('\u{2588}', ' ', f)
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> fmt::Debug for Grid2dArr<WIDTH, HEIGHT> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self._g_fmt('X', '-', f)
    }
}

impl<G: GridPrivate> GridPrivate for Box<G> {
    fn _size(&self) -> GridPoint {
        G::_size(self)
    }

    fn _frame_regulator_opt(&mut self) -> &mut Option<FrameRegulator> {
        G::_frame_regulator_opt(self)
    }

    fn _get_next_cell_unchecked_mut(&mut self, point: GridPoint) -> &mut bool {
        G::_get_next_cell_unchecked_mut(self, point)
    }

    fn _regulate_frame(&mut self) {
        G::_regulate_frame(self)
    }

    fn _g_fmt(&self, tc: char, fc: char, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        G::_g_fmt(self, tc, fc, f)
    }
}

impl<G: Grid> Grid for Box<G> {
    fn update(&mut self) {
        G::update(self);
    }

    fn get_cell_unchecked(&self, point: GridPoint) -> bool {
        G::get_cell_unchecked(self, point)
    }

    fn get_cell_unchecked_mut(&mut self, point: GridPoint) -> &mut bool {
        G::get_cell_unchecked_mut(self, point)
    }
}

/*Derived clone causing intermittent stack overflows on windows. Manually derive clone for Box instead.*/
impl<const WIDTH: usize, const HEIGHT: usize> Clone for Box<Grid2dArr<WIDTH, HEIGHT>> {
    fn clone(&self) -> Self {
        let mut ret = unsafe {
            Box::from_raw(alloc::alloc(Layout::new::<Grid2dArr<WIDTH, HEIGHT>>())
                as *mut Grid2dArr<WIDTH, HEIGHT>)
        };
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                ret.current_arr[y][x] = self.current_arr[y][x];
                ret.next_arr[y][x] = self.next_arr[y][x];
            }
        }
        ret.frame_regulator_opt = self.frame_regulator_opt;
        ret
    }
}
