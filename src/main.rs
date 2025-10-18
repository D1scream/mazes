use pacwoman::{Map, find_path};

fn main() {
    match Map::parse_from_stdin() {
        Ok(mut map) => {
            if let Some(path) = find_path(&map) {
                map.mark_path(&path);
            }
            print!("{}", map);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}

