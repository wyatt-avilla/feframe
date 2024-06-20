use std::fs;
use std::path::Path;

fn main() {
    let dist_dir = Path::new("../frontend/dist");
    if !dist_dir.exists() || fs::read_dir(dist_dir).map_or(true, |entries| entries.count() == 0) {
        println!("cargo:warning=Unable to serve frontened");
    }
}
