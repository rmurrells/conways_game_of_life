use crate::{BResult, Grid};

pub fn test<G: Grid>(grid: &mut G) -> BResult<()> {
    grid.block((1, 1))?;
    grid.bee_hive((5, 1))?;
    grid.loaf((11, 1))?;
    grid.boat((17, 1))?;
    grid.tub((22, 1))?;
    grid.blinker((28, 1))?;
    grid.toad((32, 1))?;
    grid.beacon((38, 1))?;
    grid.pulsar((45, 1))?;
    grid.penta_decathlon((64, 3))?;
    grid.lwss((26, 10))?;
    grid.mwss((35, 7))?;
    grid.hwss((35, 14))
}
