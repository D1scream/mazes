use std::collections::{VecDeque, HashSet, HashMap};
use crate::map::{Map, Position};

pub fn find_path(map: &Map) -> Option<Vec<Position>> {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    let mut parent = HashMap::new();
    
    queue.push_back(map.start);
    visited.insert(map.start);
    
    while let Some(current) = queue.pop_front() {
        if current == map.end {
            let mut path = Vec::new();
            let mut pos = current;
            
            while pos != map.start {
                path.push(pos);
                pos = parent[&pos];
            }
            path.push(map.start);
            path.reverse();
            return Some(path);
        }
        
        for neighbor in map.get_neighbors(current) {
            if !visited.contains(&neighbor) {
                visited.insert(neighbor);
                parent.insert(neighbor, current);
                queue.push_back(neighbor);
            }
        }
    }
    
    None
}
