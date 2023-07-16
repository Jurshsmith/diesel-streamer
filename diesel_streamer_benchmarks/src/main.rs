use criterion::criterion_main;

use diesel_streamer_benchmarks::benchmarks;

criterion_main! {
  benchmarks::compare_functions::fibonaccis,
}
