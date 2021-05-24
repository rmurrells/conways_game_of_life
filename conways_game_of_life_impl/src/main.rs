use conways_game_of_life_impl::{BResult, Grid, Grid2d, LinearGrid};

fn main() -> BResult<()> {
    //let mut game = LinearGrid::empty((80, 21));
    let mut game = Grid2d::empty((80, 21));

    game.block((1, 1))?;
    game.bee_hive((5, 1))?;
    game.loaf((11, 1))?;
    game.boat((17, 1))?;
    game.tub((22, 1))?;
    game.blinker((28, 1))?;
    game.toad((32, 1))?;
    game.beacon((38, 1))?;
    game.pulsar((45, 1))?;
    game.penta_decathlon((64, 3))?;

    for _ in 0..1000 {
        println!("{}", game);
        game.update();
        std::thread::sleep(std::time::Duration::from_millis(500));
    }

    Ok(())
}
