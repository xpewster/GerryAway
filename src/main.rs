use clap::Parser;
use geojson::{Feature, GeoJson, Geometry, Value};
mod analyze;
mod quickhull;
use analyze::analyze;

#[derive(Parser)]
struct Config {
    // Path to the GeoJSON file containing the districts
    json_path: String,
    propertyToFilter: String,
    filter: String,
}

fn main() {
    let args = Config::parse();
    println!("Initializing GerryAway on file {}", args.json_path);

    let content = std::fs::read_to_string(&args.json_path).expect("could not read file");
    let geojson: GeoJson = content.parse::<GeoJson>().unwrap();

    analyze(geojson, &args.propertyToFilter, &args.filter);
}
