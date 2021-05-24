use criterion::{criterion_group, criterion_main, Criterion};
use conways_game_of_life_impl::{Grid, Grid2d, LinearGrid};
use std::time::Duration;

fn criterion_benchmark(c: &mut Criterion) {
    //let mut game = LinearGrid::empty((80, 21));
    let mut game = Grid2d::empty((80, 21));

    game.block((1, 1)).unwrap();
    game.bee_hive((5, 1)).unwrap();
    game.loaf((11, 1)).unwrap();
    game.boat((17, 1)).unwrap();
    game.tub((22, 1)).unwrap();
    game.blinker((28, 1)).unwrap();
    game.toad((32, 1)).unwrap();
    game.beacon((38, 1)).unwrap();
    game.pulsar((45, 1)).unwrap();
    game.penta_decathlon((64, 3)).unwrap();
	
    let mut group = c.benchmark_group("bgroup");
    group.measurement_time(Duration::from_secs(10));
    group.bench_function("default", |b| {
        b.iter(|| {
	    game.update();
        })
    });
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
