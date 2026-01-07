# Algoritmos de búsqueda BFS WASM
Este módulo expone un buscador de caminos basado en **BFS (Breadth-First Search)**
para ser utilizado desde JavaScript vía **wasm-bindgen**.
## Descripción
- El grid se representa como un vector plano (`Vec<u8>`)
- `1` representa camino libre
- `0` representa obstáculo
- El inicio es `(0,0)`
- El destino es `(n-1, n-1)`
BFS garantiza encontrar **la ruta más corta** si existe.
```
use wasm_bindgen::prelude::*;
use std::collections::{VecDeque, HashMap};

```
Estructura expuesta a JavaScript.
Contiene la ruta final encontrada por BFS.
Si no existe ruta, estará vacía.
```
#[wasm_bindgen]
pub struct PathFinder {
    path: Vec<(usize, usize)>,
}
```

#[wasm_bindgen]
impl PathFinder {
Crea un nuevo `PathFinder` y ejecuta BFS inmediatamente.
# Parámetros
- `grid`: vector plano del grid (`n * n`)
- `size`: tamaño del grid (`n`)
# Ejemplo
```text
grid = [
  1, 1, 0,
  0, 1, 1,
  0, 1, 1
]
size = 3
```

#[wasm_bindgen(constructor)]
pub fn new(grid: Vec<u8>, size: usize) -> Self {
    let path = bfs(grid, size);
        Self { path }
    }

    Indica si existe un camino válido.
    
    # Retorna
    - `true` si BFS encontró una ruta
    - `false` en caso contrario
pub fn has_path(&self) -> bool {
        !self.path.is_empty()
}

Devuelve la ruta como un vector plano `[x1, y1, x2, y2, ...]`

pub fn path(&self) -> Vec<usize> {
    self.path
        .iter()
        .flat_map(|(x, y)| vec![*x, *y])
        .collect()
}

-

Ejecuta BFS sobre el grid.
Retorna la ruta desde `(0,0)` hasta `(n-1,n-1)`
o un vector vacío si no existe camino.


 





