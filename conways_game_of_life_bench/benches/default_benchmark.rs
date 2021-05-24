use criterion::{BenchmarkGroup, criterion_group, criterion_main, Criterion, measurement::WallTime};
use conways_game_of_life_impl::{Grid, Grid2d, LinearGrid};
use std::time::Duration;

fn init_benchmark<G: Grid>(name: &str, mut game: G, group: &mut BenchmarkGroup<'_, WallTime>) {
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
	
    group.bench_function(name, |b| {
        b.iter(|| {
	    game.update();
        })
    });
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("bgroup");
    group.measurement_time(Duration::from_secs(10));

    let size = (1000, 1000);
    init_benchmark("linear", LinearGrid::empty(size), &mut group);
    init_benchmark("2d", Grid2d::empty(size), &mut group);

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
