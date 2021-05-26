use crate::{BResult, Grid, GridPoint};

fn block<G: Grid>(grid: &mut G, (x, y): GridPoint) -> BResult<()> {
    grid.set_hline(x, 2, y, true)?;
    grid.set_hline(x, 2, y + 1, true)
}

fn bee_hive<G: Grid>(grid: &mut G, (x, y): GridPoint) -> BResult<()> {
    grid.set_hline(x + 1, 2, y, true)?;
    grid.set_cell((x, y + 1), true)?;
    grid.set_cell((x + 3, y + 1), true)?;
    grid.set_hline(x + 1, 2, y + 2, true)
}

fn loaf<G: Grid>(grid: &mut G, (x, y): GridPoint) -> BResult<()> {
    grid.set_hline(x + 1, 2, y, true)?;
    grid.set_vline(y + 1, 2, x + 3, true)?;
    grid.set_cell((x, y + 1), true)?;
    grid.set_cell((x + 1, y + 2), true)?;
    grid.set_cell((x + 2, y + 3), true)
}

fn boat<G: Grid>(grid: &mut G, (x, y): GridPoint) -> BResult<()> {
    grid.set_hline(x, 2, y, true)?;
    grid.set_cell((x, y + 1), true)?;
    grid.set_cell((x + 2, y + 1), true)?;
    grid.set_cell((x + 1, y + 2), true)
}

fn tub<G: Grid>(grid: &mut G, (x, y): GridPoint) -> BResult<()> {
    grid.set_cell((x + 1, y), true)?;
    grid.set_cell((x, y + 1), true)?;
    grid.set_cell((x + 2, y + 1), true)?;
    grid.set_cell((x + 1, y + 2), true)
}

fn blinker<G: Grid>(grid: &mut G, (x, y): GridPoint) -> BResult<()> {
    grid.set_hline(x, 3, y, true)
}

fn toad<G: Grid>(grid: &mut G, (x, y): GridPoint) -> BResult<()> {
    grid.set_hline(x + 1, 3, y, true)?;
    grid.set_hline(x, 3, y + 1, true)
}

fn beacon<G: Grid>(grid: &mut G, (x, y): GridPoint) -> BResult<()> {
    grid.block((x, y))?;
    grid.block((x + 2, y + 2))
}

fn pulsar<G: Grid>(grid: &mut G, (x, y): GridPoint) -> BResult<()> {
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

fn penta_decathlon<G: Grid>(grid: &mut G, (x, y): GridPoint) -> BResult<()> {
    grid.set_vline(y, 10, x + 1, true)?;
    grid.set_hline(x, 3, y + 2, true)?;
    grid.set_hline(x, 3, y + 7, true)?;

    grid.set_cell((x + 1, y + 2), false)?;
    grid.set_cell((x + 1, y + 7), false)
}

fn lwss<G: Grid>(grid: &mut G, (x, y): GridPoint) -> BResult<()> {
    grid.set_cell((x + 1, y), true)?;
    grid.set_cell((x + 4, y), true)?;
    grid.set_cell((x + 4, y + 2), true)?;
    grid.set_vline(y + 1, 2, x, true)?;
    grid.set_hline(x, 4, y + 3, true)
}

fn mwss<G: Grid>(grid: &mut G, (x, y): GridPoint) -> BResult<()> {
    grid.set_cell((x + 3, y), true)?;
    grid.set_cell((x + 1, y + 1), true)?;
    grid.set_cell((x + 5, y + 1), true)?;
    grid.set_cell((x + 5, y + 3), true)?;
    grid.set_vline(y + 2, 2, x, true)?;
    grid.set_hline(x, 5, y + 4, true)
}

fn hwss<G: Grid>(grid: &mut G, (x, y): GridPoint) -> BResult<()> {
    grid.set_hline(x + 3, 2, y, true)?;
    grid.set_cell((x + 1, y + 1), true)?;
    grid.set_cell((x + 6, y + 1), true)?;
    grid.set_cell((x + 6, y + 3), true)?;
    grid.set_vline(y + 2, 2, x, true)?;
    grid.set_hline(x, 6, y + 4, true)
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
