use std::{error::Error, fmt, mem, slice::{Chunks, Iter}};

pub type GridUnit = u32;
pub type GridPoint = (GridUnit, GridUnit);

fn grid_point_contained(point: GridPoint, size: GridPoint) -> bool {
    point.0 < size.0 && point.1 < size.1
}

#[derive(Debug)]
pub struct OutOfBounds{point: GridPoint, size: GridPoint}
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
            Err(OutOfBounds{point, size})
        }
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
        if x != 0 && y != 0 {
            if self.get_cell((x - 1, y - 1)) {
                counter += 1;
            }
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

    fn block(&mut self, (x, y): GridPoint) -> BResult<()> {
        self.set_hline(x, 2, y, true)?;
        self.set_hline(x, 2, y + 1, true)
    }

    fn bee_hive(&mut self, (x, y): GridPoint) -> BResult<()> {
        self.set_hline(x + 1, 2, y, true)?;
        self.set_cell((x, y + 1), true)?;
        self.set_cell((x + 3, y + 1), true)?;
        self.set_hline(x + 1, 2, y + 2, true)
    }

    fn loaf(&mut self, (x, y): GridPoint) -> BResult<()> {
        self.set_hline(x + 1, 2, y, true)?;
        self.set_vline(y + 1, 2, x + 3, true)?;
        self.set_cell((x, y + 1), true)?;
        self.set_cell((x + 1, y + 2), true)?;
        self.set_cell((x + 2, y + 3), true)
    }

    fn boat(&mut self, (x, y): GridPoint) -> BResult<()> {
        self.set_hline(x, 2, y, true)?;
        self.set_cell((x, y + 1), true)?;
        self.set_cell((x + 2, y + 1), true)?;
        self.set_cell((x + 1, y + 2), true)
    }

    fn tub(&mut self, (x, y): GridPoint) -> BResult<()> {
        self.set_cell((x + 1, y), true)?;
        self.set_cell((x, y + 1), true)?;
        self.set_cell((x + 2, y + 1), true)?;
        self.set_cell((x + 1, y + 2), true)
    }

    fn blinker(&mut self, (x, y): GridPoint) -> BResult<()> {
        self.set_hline(x, 3, y, true)
    }

    fn toad(&mut self, (x, y): GridPoint) -> BResult<()> {
        self.set_hline(x + 1, 3, y, true)?;
        self.set_hline(x, 3, y + 1, true)
    }

    fn beacon(&mut self, (x, y): GridPoint) -> BResult<()> {
        self.block((x, y))?;
        self.block((x + 2, y + 2))
    }

    fn pulsar(&mut self, (x, y): GridPoint) -> BResult<()> {
        let length = 3;

        self.set_hline(x + 2, length, y, true)?;
        self.set_hline(x + 2, length, y + 5, true)?;
        self.set_hline(x + 2, length, y + 7, true)?;
        self.set_hline(x + 2, length, y + 12, true)?;

        self.set_hline(x + 8, length, y, true)?;
        self.set_hline(x + 8, length, y + 5, true)?;
        self.set_hline(x + 8, length, y + 7, true)?;
        self.set_hline(x + 8, length, y + 12, true)?;

        self.set_vline(y + 2, length, x, true)?;
        self.set_vline(y + 2, length, x + 5, true)?;
        self.set_vline(y + 2, length, x + 7, true)?;
        self.set_vline(y + 2, length, x + 12, true)?;

        self.set_vline(y + 8, length, x, true)?;
        self.set_vline(y + 8, length, x + 5, true)?;
        self.set_vline(y + 8, length, x + 7, true)?;
        self.set_vline(y + 8, length, x + 12, true)
    }

    fn penta_decathlon(&mut self, (x, y): GridPoint) -> BResult<()> {
	self.set_vline(y, 10, x + 1, true)?;
	self.set_hline(x, 3, y+2, true)?;
	self.set_hline(x, 3, y+7, true)?;

	self.set_cell((x + 1, y+2), false)?;
	self.set_cell((x + 1, y+7), false)
    }
}

pub trait GridIter<'a>
where 
    Self::I: IntoIterator,
    <Self::I as IntoIterator>::Item: IntoIterator<Item = &'a bool>,
{
    type I;
    fn iter(&'a self) -> Self::I;
}

pub struct LinearGrid {
    size: GridPoint,
    current_vec: Vec<bool>,
    next_vec: Vec<bool>,
}

impl LinearGrid {
    pub fn empty(size: GridPoint) -> Self {
        let next_vec = vec![false; (size.0 * size.1) as usize];
        Self {
            size,
            current_vec: next_vec.clone(),
            next_vec,
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

    fn get_cell_unchecked(&self, point: GridPoint) -> bool {
        self.current_vec[self.get_index(point)]
    }

    fn get_cell_unchecked_mut(&mut self, point: GridPoint) -> &mut bool {
        let index = self.get_index(point);
        &mut self.current_vec[index]
    }

    fn update(&mut self) {
        for x in 0..self.size.0 {
            for y in 0..self.size.1 {
		let index = self.get_index((x, y));
		self.next_vec[index] = self.next_cell_state((x, y));
            }
        }
        mem::swap(&mut self.current_vec, &mut self.next_vec);
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

impl<'a> GridIter<'a> for LinearGrid {
    type I = Chunks<'a, bool>;
    fn iter(&'a self) -> Self::I {
        self.current_vec.chunks(self.size.0 as usize)
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
}

impl Grid2d {
    pub fn empty(size: GridPoint) -> Self {
	let next_vec = vec![vec![false; size.0 as usize]; size.1 as usize];
	Self {
            size,
	    current_vec: next_vec.clone(),
	    next_vec,
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

    fn get_cell_unchecked(&self, (x, y): GridPoint) -> bool {
        self.current_vec[y as usize][x as usize]
    }

    fn get_cell_unchecked_mut(&mut self, (x, y): GridPoint) -> &mut bool {
        &mut self.current_vec[y as usize][x as usize]
    }

    fn update(&mut self) {
        for x in 0..self.size.0 {
            for y in 0..self.size.1 {
		self.next_vec[y as usize][x as usize] = self.next_cell_state((x, y));
            }
        }
        mem::swap(&mut self.current_vec, &mut self.next_vec);
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

impl<'a> GridIter<'a> for Grid2d {
    type I = Iter<'a, Vec<bool>>;
    fn iter(&'a self) -> Self::I {
        self.current_vec.iter()
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
