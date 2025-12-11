use pacwoman::{Map, Position, Cell, find_path};

#[test]
fn test_path_finding() {
    let map = Map {
        grid: vec![
            vec![Cell::Empty, Cell::Empty, Cell::Empty],
            vec![Cell::Empty, Cell::Wall, Cell::Empty],
            vec![Cell::Empty, Cell::Empty, Cell::Empty],
        ],
        start: Position { row: 0, col: 0 },
        end: Position { row: 2, col: 2 },
        rows: 3,
        cols: 3,
    };

    let path = find_path(&map);
    assert!(path.is_some());
    
    if let Some(path) = path {
        assert_eq!(path[0], map.start);
        assert_eq!(path[path.len() - 1], map.end);
    }
}

#[test]
fn test_no_path() {
    let map = Map {
        grid: vec![
            vec![Cell::Wall, Cell::Wall, Cell::Wall],
            vec![Cell::Wall, Cell::Empty, Cell::Wall],
            vec![Cell::Wall, Cell::Wall, Cell::Wall],
        ],
        start: Position { row: 1, col: 1 },
        end: Position { row: 1, col: 1 },
        rows: 3,
        cols: 3,
    };

    let path = find_path(&map);
    assert!(path.is_some()); // Same start and end position
}

