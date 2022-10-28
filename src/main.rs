use moka::future::Cache;

// alternatively, you can reproduce the problem with:
// use moka::sync::Cache;

#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

fn main() {
    // If you comment out this line, the program no longer hangs:
    let _profiler = dhat::Profiler::new_heap();

    println!("Starting");

    let _cache: Cache<String, String> = Cache::builder().max_capacity(1).build();
    println!("Done!");
}
