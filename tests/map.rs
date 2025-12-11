use pacwoman::{Map, Position, Cell};

#[test]
fn test_map_parsing() {
    let map = Map {
        grid: vec![
            vec![Cell::Wall, Cell::Wall, Cell::Empty, Cell::Empty, Cell::Empty, Cell::Wall],
            vec![Cell::Wall, Cell::Empty, Cell::Empty, Cell::Wall, Cell::Start, Cell::Empty, Cell::Wall],
            vec![Cell::Wall, Cell::Empty, Cell::Empty, Cell::End, Cell::Wall, Cell::Wall, Cell::Empty],
            vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Wall, Cell::Empty, Cell::Empty, Cell::Empty],
        ],
        start: Position { row: 1, col: 4 },
        end: Position { row: 2, col: 3 },
        rows: 4,
        cols: 7,
    };

    assert_eq!(map.start, Position { row: 1, col: 4 });
    assert_eq!(map.end, Position { row: 2, col: 3 });
}

#[test]
fn test_toroidal_neighbors() {
    let map = Map {
        grid: vec![
            vec![Cell::Empty, Cell::Empty, Cell::Empty],
            vec![Cell::Empty, Cell::Empty, Cell::Empty],
            vec![Cell::Empty, Cell::Empty, Cell::Empty],
        ],
        start: Position { row: 0, col: 0 },
        end: Position { row: 0, col: 0 },
        rows: 3,
        cols: 3,
    };

    let neighbors = map.get_neighbors(Position { row: 0, col: 0 });
    assert_eq!(neighbors.len(), 4);
    
    let expected = vec![
        Position { row: 2, col: 0 }, // up (toroidal)
        Position { row: 1, col: 0 }, // down
        Position { row: 0, col: 2 }, // left (toroidal)
        Position { row: 0, col: 1 }, // right
    ];
    
    for expected_pos in expected {
        assert!(neighbors.contains(&expected_pos));
    }
}

