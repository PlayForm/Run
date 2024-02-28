extern crate glob;
extern crate globset;
extern crate ignore;
extern crate walkdir;

use std::{fs, time::Instant};

fn main() {
	// Define your glob pattern
	let pattern = "*.md";

	// Define the directory to search in
	let dir = ".";

	// Benchmark glob crate
	let start = Instant::now();
	let glob_results = glob::glob(&format!("{}/{}", dir, pattern))
		.unwrap()
		.collect::<Result<Vec<_>, _>>()
		.unwrap();
	let glob_elapsed = start.elapsed();

	// Benchmark ignore crate
	let start = Instant::now();
	let ignore_results = ignore::WalkBuilder::new("./*.md")
		.build()
		.filter_map(|e| e.ok())
		.map(|e| e.into_path())
		.collect::<Vec<_>>();
	let ignore_elapsed = start.elapsed();

	// Benchmark walkdir crate
	let start = Instant::now();
	let walkdir_results = walkdir::WalkDir::new(dir)
		.into_iter()
		.filter_map(|e| e.ok())
		.filter(|e| e.file_name().to_str().unwrap().ends_with(".md"))
		.map(|e| e.path().to_owned())
		.collect::<Vec<_>>();
	let walkdir_elapsed = start.elapsed();

	// Benchmark globset crate
	let start = Instant::now();
	let builder =
		globset::GlobBuilder::new(pattern).case_insensitive(true).build();

	let globset_results: Vec<_> = fs::read_dir(dir)
		.unwrap()
		.filter_map(|entry| entry.ok())
		.map(|entry| entry.path())
		.collect();
	let globset_elapsed = start.elapsed();

	// Print results
	println!("Results:");
	println!("glob crate: {} results, took {:?}", glob_results.len(), glob_elapsed);
	println!("ignore crate: {} results, took {:?}", ignore_results.len(), ignore_elapsed);
	println!("walkdir crate: {} results, took {:?}", walkdir_results.len(), walkdir_elapsed);
	println!("globset crate: {} results, took {:?}", globset_results.len(), globset_elapsed);
}
