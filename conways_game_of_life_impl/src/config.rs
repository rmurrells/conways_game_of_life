use crate::{BResult, Grid, GridPoint};
use rand::{rngs::StdRng, Rng, SeedableRng};

pub fn block<G: Grid>(grid: &mut G, (x, y): GridPoint) -> BResult<()> {
    grid.set_hline(x, 2, y, true)?;
    grid.set_hline(x, 2, y + 1, true)
}

pub fn bee_hive<G: Grid>(grid: &mut G, (x, y): GridPoint) -> BResult<()> {
    grid.set_hline(x + 1, 2, y, true)?;
    grid.set_cell((x, y + 1), true)?;
    grid.set_cell((x + 3, y + 1), true)?;
    grid.set_hline(x + 1, 2, y + 2, true)
}

pub fn loaf<G: Grid>(grid: &mut G, (x, y): GridPoint) -> BResult<()> {
    grid.set_hline(x + 1, 2, y, true)?;
    grid.set_vline(y + 1, 2, x + 3, true)?;
    grid.set_cell((x, y + 1), true)?;
    grid.set_cell((x + 1, y + 2), true)?;
    grid.set_cell((x + 2, y + 3), true)
}

pub fn boat<G: Grid>(grid: &mut G, (x, y): GridPoint) -> BResult<()> {
    grid.set_hline(x, 2, y, true)?;
    grid.set_cell((x, y + 1), true)?;
    grid.set_cell((x + 2, y + 1), true)?;
    grid.set_cell((x + 1, y + 2), true)
}

pub fn tub<G: Grid>(grid: &mut G, (x, y): GridPoint) -> BResult<()> {
    grid.set_cell((x + 1, y), true)?;
    grid.set_cell((x, y + 1), true)?;
    grid.set_cell((x + 2, y + 1), true)?;
    grid.set_cell((x + 1, y + 2), true)
}

pub fn blinker<G: Grid>(grid: &mut G, (x, y): GridPoint) -> BResult<()> {
    grid.set_hline(x, 3, y, true)
}

pub fn toad<G: Grid>(grid: &mut G, (x, y): GridPoint) -> BResult<()> {
    grid.set_hline(x + 1, 3, y, true)?;
    grid.set_hline(x, 3, y + 1, true)
}

pub fn beacon<G: Grid>(grid: &mut G, (x, y): GridPoint) -> BResult<()> {
    block(grid, (x, y))?;
    block(grid, (x + 2, y + 2))
}

pub fn pulsar<G: Grid>(grid: &mut G, (x, y): GridPoint) -> BResult<()> {
    let length = 3;

    grid.set_hline(x + 2, length, y, true)?;
    grid.set_hline(x + 2, length, y + 5, true)?;
    grid.set_hline(x + 2, length, y + 7, true)?;
    grid.set_hline(x + 2, length, y + 12, true)?;

    grid.set_hline(x + 8, length, y, true)?;
    grid.set_hline(x + 8, length, y + 5, true)?;
    grid.set_hline(x + 8, length, y + 7, true)?;
    grid.set_hline(x + 8, length, y + 12, true)?;

    grid.set_vline(y + 2, length, x, true)?;
    grid.set_vline(y + 2, length, x + 5, true)?;
    grid.set_vline(y + 2, length, x + 7, true)?;
    grid.set_vline(y + 2, length, x + 12, true)?;

    grid.set_vline(y + 8, length, x, true)?;
    grid.set_vline(y + 8, length, x + 5, true)?;
    grid.set_vline(y + 8, length, x + 7, true)?;
    grid.set_vline(y + 8, length, x + 12, true)
}

pub fn penta_decathlon<G: Grid>(grid: &mut G, (x, y): GridPoint) -> BResult<()> {
    grid.set_vline(y, 10, x + 1, true)?;
    grid.set_hline(x, 3, y + 2, true)?;
    grid.set_hline(x, 3, y + 7, true)?;

    grid.set_cell((x + 1, y + 2), false)?;
    grid.set_cell((x + 1, y + 7), false)
}

pub fn glider<G: Grid>(grid: &mut G, (x, y): GridPoint) -> BResult<()> {
    grid.set_cell((x + 1, y), true)?;
    grid.set_cell((x + 2, y + 1), true)?;
    grid.set_hline(x, 3, y + 2, true)
}

pub fn lwss<G: Grid>(grid: &mut G, (x, y): GridPoint) -> BResult<()> {
    grid.set_cell((x + 1, y), true)?;
    grid.set_cell((x + 4, y), true)?;
    grid.set_cell((x + 4, y + 2), true)?;
    grid.set_vline(y + 1, 2, x, true)?;
    grid.set_hline(x, 4, y + 3, true)
}

pub fn mwss<G: Grid>(grid: &mut G, (x, y): GridPoint) -> BResult<()> {
    grid.set_cell((x + 3, y), true)?;
    grid.set_cell((x + 1, y + 1), true)?;
    grid.set_cell((x + 5, y + 1), true)?;
    grid.set_cell((x + 5, y + 3), true)?;
    grid.set_vline(y + 2, 2, x, true)?;
    grid.set_hline(x, 5, y + 4, true)
}

pub fn hwss<G: Grid>(grid: &mut G, (x, y): GridPoint) -> BResult<()> {
    grid.set_hline(x + 3, 2, y, true)?;
    grid.set_cell((x + 1, y + 1), true)?;
    grid.set_cell((x + 6, y + 1), true)?;
    grid.set_cell((x + 6, y + 3), true)?;
    grid.set_vline(y + 2, 2, x, true)?;
    grid.set_hline(x, 6, y + 4, true)
}

pub fn random<G: Grid>(grid: &mut G, mut density: f64) -> BResult<()> {
    density = density.clamp(0., 1.);
    let mut rng = <StdRng as SeedableRng>::seed_from_u64(0);
    grid.try_inspect_mut(|point, grid| {
        if rng.gen::<f64>() <= density {
            grid.set_cell(point, true)
        } else {
            Ok(())
        }
    })
}

pub fn test<G: Grid>(grid: &mut G) -> BResult<()> {
    block(grid, (1, 1))?;
    bee_hive(grid, (5, 1))?;
    loaf(grid, (11, 1))?;
    boat(grid, (17, 1))?;
    tub(grid, (22, 1))?;
    blinker(grid, (28, 1))?;
    toad(grid, (32, 1))?;
    beacon(grid, (38, 1))?;
    pulsar(grid, (45, 1))?;
    penta_decathlon(grid, (64, 3))?;
    lwss(grid, (26, 10))?;
    mwss(grid, (35, 7))?;
    hwss(grid, (35, 14))
}
