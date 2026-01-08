use std::collections::{HashMap, VecDeque};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct PathFinder {
    path: Vec<(usize, usize)>,
}

#[wasm_bindgen]
impl PathFinder {
    #[wasm_bindgen(constructor)]
    pub fn new(grid: Vec<u8>, size: usize) -> Self {
        let path = bfs(grid, size);
        Self { path }
    }

    pub fn has_path(&self) -> bool {
        !self.path.is_empty()
    }
    pub fn path(&self) -> Vec<usize> {
        self.path.iter().flat_map(|(x, y)| vec![*x, *y]).collect()
    }
}

fn bfs(grid: Vec<u8>, n: usize) -> Vec<(usize, usize)> {
    if grid[0] == 0 || grid[n * n - 1] == 0 {
        return vec![];
    }

    let mut queue = VecDeque::new();
    let mut visited = vec![false; n * n];
    let mut parent: HashMap<(usize, usize), (usize, usize)> = HashMap::new();

    queue.push_back((0, 0));
    visited[0] = true;

    let dirs = [(1, 0), (0, 1), (-1, 0), (0, -1)];

    while let Some((x, y)) = queue.pop_front() {
        if (x, y) == (n - 1, n - 1) {
            return build_path(parent, (x, y));
        }

        for (dx, dy) in dirs {
            let nx = x as isize + dx;
            let ny = y as isize + dy;

            if nx >= 0 && ny >= 0 {
                let (nx, ny) = (nx as usize, ny as usize);
                let idx = nx * n + ny;

                if nx < n && ny < n && grid[idx] == 1 && !visited[idx] {
                    visited[idx] = true;
                    parent.insert((nx, ny), (x, y));
                    queue.push_back((nx, ny));
                }
            }
        }
    }

    vec![]
}

fn build_path(
    parent: HashMap<(usize, usize), (usize, usize)>,
    mut end: (usize, usize),
) -> Vec<(usize, usize)> {
    let mut path = vec![end];
    while let Some(&p) = parent.get(&end) {
        end = p;
        path.push(end);
    }
    path.reverse();
    path
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test: camino simple sin obstáculos
    #[test]
    fn path_exists_simple() {
        let grid = vec![1, 1, 1, 1];
        let path = bfs(grid, 2);
        assert!(!path.is_empty());
    }

    /// Test: inicio bloqueado
    #[test]
    fn no_path_start_blocked() {
        let grid = vec![0, 1, 1, 1];
        let path = bfs(grid, 2);
        assert!(path.is_empty());
    }

    /// Test: destino bloqueado
    #[test]
    fn no_path_end_blocked() {
        let grid = vec![1, 1, 1, 0];
        let path = bfs(grid, 2);
        assert!(path.is_empty());
    }

    /// Test: grid 3x3 con ruta válida mínima
    #[test]
    fn path_exists_complex() {
        let grid = vec![1, 1, 0, 0, 1, 1, 0, 1, 1];
        let path = bfs(grid, 3);
        assert_eq!(path.first(), Some(&(0, 0)));
        assert_eq!(path.last(), Some(&(2, 2)));
    }
}
